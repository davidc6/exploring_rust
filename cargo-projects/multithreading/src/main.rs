use crate::{counter::most_frequent_char, flag::stop_flag};
use counter::{increment_counter, increment_counter_atomic};
use mutex::simple_mutex;
use progress_updater::{progress_updater, progress_updater_parking, progress_updater_scoped};

mod counter;
mod flag;
mod mutex;
mod progress_updater;

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
    simple_mutex();
}
