// Cell

use std::cell::Cell;

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
