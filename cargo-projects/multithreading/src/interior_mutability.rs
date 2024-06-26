use std::{
    cell::{Cell, RefCell, UnsafeCell},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::AtomicBool,
};

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

// ------------------
//
// Cell
//  - Enables to move values in and out of the cell often used for simple types
//  - A mutable reference to the inner value cannot be obtained
//  - A direct value cannot be obtained without replacing it with another one
//
// ------------------

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
#[derive(Debug)]
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

// ------------------
//
// RefCell
//  - A type that enables temporary, exclusive, mutable access to the inner value.
//  - borrow()     - immutable reference to the inner value
//  - borrow_mut() - mutable borrow
//
// ------------------

pub fn refcell_fn() {
    // let x = 1;
    // this errors since we are attempting to borrow immutable as mutable
    // let y = &mut x;

    // Interior mutability is allowed using RefCell
    let x = RefCell::new(1);
    x.replace(7);

    println!("{:?}", x.into_inner());
}

// We don't actually have to do this since UnsafeCell is !Sync
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

#[derive(Clone, Copy, Debug)]
pub enum RefCellTypeState {
    NotShared,
    Shared(usize),
    Exclusive,
}

#[derive(Debug)]
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
    pub fn borrow(&self) -> Ref<'_, T> {
        match self.refs.get() {
            RefCellTypeState::NotShared => {
                self.refs.set(RefCellTypeState::Shared(1));

                // SAFETY: Not exclusive shared hence it's safe to share the reference.
                Some(Ref { ref_cell: self })
            }
            RefCellTypeState::Shared(shared_count) => {
                self.refs.set(RefCellTypeState::Shared(shared_count + 1));

                // SAFETY: Not exclusive shared hence it's safe to share the reference.
                Some(Ref { ref_cell: self })
            }
            RefCellTypeState::Exclusive => None,
        }
        .unwrap()
    }

    // Borrow mutably/exclusively.
    //
    // If attempt is made to borrow exclusively once again,
    // a None will be returned.
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        match self.refs.get() {
            RefCellTypeState::NotShared => {
                self.refs.set(RefCellTypeState::Exclusive);

                // SAFETY: Not shared at all hence it's safe to give out an exclusive reference.
                // Some(unsafe { &mut *self.value.get() })
                Some(RefMut { ref_cell: self })
            }
            RefCellTypeState::Shared(shared_count) => {
                self.refs.set(RefCellTypeState::Shared(shared_count + 1));

                None
            }
            RefCellTypeState::Exclusive => None,
        }
        .unwrap()
    }
}

#[derive(Debug)]
pub struct Ref<'refcell, T> {
    ref_cell: &'refcell RefCellType<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.ref_cell.refs.get() {
            RefCellTypeState::Exclusive | RefCellTypeState::NotShared => unreachable!(),
            RefCellTypeState::Shared(1) => {
                self.ref_cell.refs.set(RefCellTypeState::NotShared);
            }
            RefCellTypeState::Shared(n) => {
                self.ref_cell.refs.set(RefCellTypeState::Shared(n - 1));
            }
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    // SAFETY: Similar to DeferMut
    fn deref(&self) -> &T {
        unsafe { &*self.ref_cell.value.get() }
    }
}

impl<T> DerefMut for Ref<'_, T> {
    // SAFETY: Ref gets created when there are no other references.
    //
    // When an exclusive reference is given out and the state is set to Exclusive,
    // no future references are given out.
    //
    // Since this reference is exclusive we can mutably dereference it without any problems.
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ref_cell.value.get() }
    }
}

pub struct RefMut<'refcell, T> {
    ref_cell: &'refcell RefCellType<T>,
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.ref_cell.refs.get() {
            RefCellTypeState::Exclusive => {
                self.ref_cell.refs.set(RefCellTypeState::NotShared);
            }
            RefCellTypeState::NotShared | RefCellTypeState::Shared(_) => unreachable!(),
        }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    // SAFETY: Similar to DeferMut
    fn deref(&self) -> &T {
        unsafe { &*self.ref_cell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    // SAFETY: Ref gets created when there are no other references.
    //
    // When an exclusive reference is given out and the state is set to Exclusive,
    // no future references are given out.
    //
    // Since this reference is exclusive we can mutably dereference it without any problems.
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ref_cell.value.get() }
    }
}
