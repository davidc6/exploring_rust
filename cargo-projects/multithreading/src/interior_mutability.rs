// Cell

use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell, UnsafeCell},
    marker::PhantomData,
    sync::atomic::AtomicBool,
};

fn cell_f_test() {
    println!("Function ran");
}

// Cell is great to use on simple copy types and cheap to copy.
// Usually used to store some thread-local state (flag, variable etc.).
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
            value: UnsafeCell::new(value), //
        }
    }

    // On the first lock(), locks the data.
    // On consecutive lock() spins while the the data is still locked.
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

    // unlock() the data
    pub fn unlock(&self) {
        self.is_locked
            .store(false, std::sync::atomic::Ordering::Release);
    }
}

// In order to share the data between threads we need to implement Sync on SpinLock.
// By doing this we tell the compiler that it is actually safe to
// share data between threads but we must only limit to types that are safe to send (Send).
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

// Rust uses "inherited mutability", so that you can only mutate value

// Cell
// Pointer aliasing - a single memory location is accessible through different symbolic names
// (a name that uniquely identifies a specific entity) in the program.
// Memory aliasing - a single memory location can be accessed by different pointers.
// Cell should not be shared across threads since this will introduce a race condition,
// where multiple threads will race to update a Cell which will end up in an incorrect value.
// So in other words there is no protection against multiple threads attempting to mutate concurrently.
pub fn cell_fn() {
    let c = Cell::new(1);
}

// Usually used within thread locals, local state
// The only way in Rust to go from a a shared reference to an exclusive reference,
// is by using an UnsafeCell, no casting is allowed.
pub struct CellType<T> {
    value: UnsafeCell<T>,
}

impl<T> CellType<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    // There are no references, so we don't have to invalidate any
    pub fn set(&self, value: T) {
        // Get a mutable (raw, exclusive) pointer to the wrapped value
        //
        // SAFETY: nothing else concurrently mutates this (i.e. !Sync)
        // SAFETY: no references are being given out
        unsafe {
            *self.value.get() = value;
        }
    }

    // get() gives out a copy of the value
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // self.value.get_mut()
        unsafe { *self.value.get() }
    }
}

// We don't actually have to do this since UnsafaCell is !Sync
// impl<T> !Sync for CellType<T> {}

// If we use a reference instead of a Copy here's what might happen
// pub fn cell_test_1() {
// Cell points to String "1"
// This memory should be deallocated - String::from("1")
// let a = CellType::new(String::from("1"));
// Get a reference to the string which points to "1"
// Deallocator does not yet release the memory
// let first = a.get();
// println!("{:?}", first);
// we allocate new value but the pointer is the same
// and this should not be ok
// a.set(String::from("2"));
// println!("{:?}", first);
// }

#[derive(Clone, Copy)]
pub enum RefCellTypeState {
    NotShared,
    Shared(usize),
    Exclusive,
}

pub struct RefCellType<T> {
    value: UnsafeCell<T>,
    refs: CellType<RefCellTypeState>,
}

impl<T> RefCellType<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            refs: CellType::new(RefCellTypeState::NotShared),
        }
    }

    // Borrow shared reference.A
    //
    // If already borrowed exclusively then return None.
    pub fn borrow(&self) -> Option<&T> {
        match self.refs.get() {
            RefCellTypeState::NotShared => {
                self.refs.set(RefCellTypeState::Shared(1));

                // SAFETY: Not exclusive shared hence it's safe to share the reference.
                Some(unsafe { &*self.value.get() })
            }
            RefCellTypeState::Shared(shared_count) => {
                self.refs.set(RefCellTypeState::Shared(shared_count + 1));

                // SAFETY: Not exclusive shared hence it's safe to share the reference.
                Some(unsafe { &*self.value.get() })
            }
            RefCellTypeState::Exclusive => None,
        }
    }

    // Borrow mutably/exclusively.A
    //
    // If attempt is made to borrow exclusively once again,
    // a None will be returned.
    pub fn borrow_mut(&self) -> Option<&mut T> {
        match self.refs.get() {
            RefCellTypeState::NotShared => {
                self.refs.set(RefCellTypeState::Exclusive);

                // SAFETY: Not shared at all hence it's safe to give out an exclusive reference.
                Some(unsafe { &mut *self.value.get() })
            }
            RefCellTypeState::Shared(shared_count) => {
                self.refs.set(RefCellTypeState::Shared(shared_count + 1));

                None
            }
            RefCellTypeState::Exclusive => None,
        }
    }
}
