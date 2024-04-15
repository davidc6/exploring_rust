use crate::counter::most_frequent_char;
use counter::{increment_counter, increment_counter_atomic};

mod counter;

fn main() {
    // let s = String::from("hello, this is a message for the future");
    // let str_section = most_frequent_char(s, 6);
    // println!("{}", str_section);

    increment_counter();
    increment_counter_atomic();
}
