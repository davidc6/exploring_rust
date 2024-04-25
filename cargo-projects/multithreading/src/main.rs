use std::{
    cell::{Cell, RefCell},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    counter::most_frequent_char, flag::stop_flag, interior_mutability::RefCellType,
    spin_lock::SpinLock,
};
use counter::{increment_counter, increment_counter_atomic};
use interior_mutability::{cell_f, refcell_fn};
use mutex::simple_mutex;
use progress_updater::{progress_updater, progress_updater_parking, progress_updater_scoped};

mod counter;
mod flag;
mod interior_mutability;
mod mutex;
mod progress_updater;
mod refcell_example_1;
mod spin_lock;

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

    // ------------
    //
    // SpinLock
    //
    // ------------
    let spin_lock = Arc::new(SpinLock::new(6));
    let mut handles = vec![];

    let spin_lock_clone = spin_lock.clone();
    println!("Processing on {:?}", thread::current().id());
    spin_lock_clone.lock();

    let spin_lock_clone_2 = spin_lock.clone();
    let j_1 = thread::spawn(move || {
        spin_lock_clone_2.lock();
        println!("Processing on {:?}", thread::current().id());
        thread::sleep(Duration::from_secs(2));
        spin_lock_clone_2.unlock();
    });
    handles.push(j_1);

    let spin_lock_clone_3 = spin_lock.clone();
    let j_2 = thread::spawn(move || {
        spin_lock_clone_3.lock();
        println!("Processing on {:?}", thread::current().id());
        thread::sleep(Duration::from_secs(2));
        spin_lock_clone_3.unlock();
    });
    handles.push(j_2);

    thread::sleep(Duration::from_secs(2));
    spin_lock_clone.unlock();

    for handle in handles {
        handle.join().unwrap();
    }

    // RefCell
    // refcell_fn();

    // Cell
    // cell_test_1();

    // Cell
    // let cell_one = Cell::new(1);
    // Get a copy of the value
    // let cell_one_value = cell_one.get();
    // println!("{:?}", cell_one_value);
    // cell_one.set(3);
    // println!("{:?}", cell_one_value);

    // RefCell
    // Dereferencing a String won't work since String is not a Copy type
    // let rc = RefCellType::new(String::from("hello"));
    // Dereferencing a copyable type works fine since it's a copyable type
    // let rc = RefCellType::new(1);
    // let value = *rc.borrow();
    // println!("RefCellType value deref {:?}", value);

    // let rc_2 = RefCellType::new(2);
    // let value = *rc_2.borrow_mut();

    // println!("RefCellType value deref {:?}", value);

    // println!("RefCell {:?}", *rc.borrow());
    // let rc_real = RefCell::new(String::from("a"));
    // let rc_real_deref = *rc_real.borrow();
}
