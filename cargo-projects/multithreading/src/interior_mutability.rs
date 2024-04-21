// Cell

use std::{
    borrow::Borrow,
    cell::{Cell, RefCell, UnsafeCell},
    marker::PhantomData,
    sync::atomic::AtomicBool,
};

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
pub struct SpinLock<T> {
    is_locked: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            is_locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&mut self) -> &mut T {
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
        unsafe { &mut *self.value.get() }
    }

    pub fn unlock(&self) {
        self.is_locked
            .store(false, std::sync::atomic::Ordering::Release);
    }
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

// RefCell
pub fn refcell_fn() {
    // let x = 1;
    // this errors since we are attempting to borrow immutable as mutable
    // let y = &mut x;

    // Interior mutability is allowed using RefCell
    let x = RefCell::new(1);
    x.replace(7);

    println!("{:?}", x.into_inner());
}
