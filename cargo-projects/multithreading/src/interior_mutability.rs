// Cell

use std::{cell::Cell, marker::PhantomData, sync::atomic::AtomicBool};

fn cell_f_test() {
    println!("Function ran");
}

pub fn cell_f(first_cell: &Cell<u8>, second_cell: &Cell<u8>) {
    let initial = first_cell.get();
    second_cell.set(second_cell.get() + 1);

    let modified = first_cell.get();

    if initial != modified {
        cell_f_test();
    }
}

// PhantomData is zero-sized and is treated as just Cell
// Since Cell is not Sync neither is SomeStructure
struct SomeStructure {
    count: u8,
    _unsync: PhantomData<Cell<()>>,
}

// Pointer type does not implement neither Send nor Sync traits.
// These can be implemented manually.
struct SomeOtherStructure {
    pointer: *mut u8,
}

unsafe impl Send for SomeOtherStructure {}
unsafe impl Sync for SomeOtherStructure {}

// Spin lock
// Keeps on trying to lock an already locked Mutex.
// This pattern works well when Mutex is locked only briefly,
// even though it might be a bit resourceful but performant.
pub struct SpinLock {
    is_locked: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            is_locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        // Check/compare if current value is false sets to true
        // Or if current value is true, keep on trying to lock it.
        while self
            .is_locked
            .compare_exchange_weak(
                false,
                true,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            )
            .is_err()
        {
            std::hint::spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.is_locked
            .store(false, std::sync::atomic::Ordering::Relaxed);
    }
}
