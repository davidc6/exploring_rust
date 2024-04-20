// Cell

use std::{cell::Cell, marker::PhantomData};

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
