use std::cell::Cell;

use crate::{counter::most_frequent_char, flag::stop_flag};
use counter::{increment_counter, increment_counter_atomic};
use interior_mutability::{cell_f, refcell_fn, SpinLock};
use mutex::simple_mutex;
use progress_updater::{progress_updater, progress_updater_parking, progress_updater_scoped};

mod counter;
mod flag;
mod interior_mutability;
mod mutex;
mod progress_updater;
mod refcell_example_1;

fn main() {
    // let s = String::from("hello, this is a message for the future");
    // let str_section = most_frequent_char(s, 6);
    // println!("{}", str_section);

    // increment_counter();
    // increment_counter_atomic();

    // stop_flag();

    // progress_updater();

    // progress_updater_scoped();

    // progress_updater_parking();

    // Mutex
    // simple_mutex();

    // Interior mutability
    // let first_cell = Cell::new(8);
    // let second_cell = Cell::new(2);
    // cell_f(&first_cell, &second_cell);

    // SpinLock
    // let mut spin_lock = SpinLock::new(6);
    // spin_lock.lock();
    // spin_lock.unlock();

    // spin_lock.lock();
    // spin_lock.unlock();

    // RefCell
    refcell_fn();
}
