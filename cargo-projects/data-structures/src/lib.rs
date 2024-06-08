mod errors;

use crate::errors::run_error;
use std::collections::VecDeque;

fn from_string_to_slice_and_back() -> String {
    let string = String::from("hey");
    let string_slice = &string[1..3];
    string_slice.to_owned()
}

fn some_str<'a>() -> &'a str {
    let result;

    {
        let a = "one";
        // let a = "one".to_owned() // this will fail since a will be dropped at the end of this scope
        let b = "four";
        result = longest(a, b);
    }

    result
}

fn longest<'a>(str_one: &'a str, str_two: &'a str) -> &'a str {
    if str_one.len() > str_two.len() {
        str_one
    } else {
        str_two
    }
}

fn some_vec() -> Vec<i32> {
    let mut v = vec![1, 2];

    v.push(3);

    v
}

fn some_devec() -> VecDeque<i32> {
    let mut v = VecDeque::new();

    v.push_back(1);
    v.push_back(2);

    v
}

pub fn run_all() {
    let longest_str = some_str();
    println!("{:?}", longest_str);

    let ordinary_vec = some_vec();
    println!("{:?}", ordinary_vec);

    let deq_vec = some_devec().pop_front();
    println!("{:?}", deq_vec);

    let other_str = from_string_to_slice_and_back();
    println!("{:?}", other_str);

    let some_err = run_error(1);
    println!("{:?}", some_err);
}

#[cfg(test)]
mod tests {
    use errors::{FirstCustomErrors, SecondCustomErrors};

    use super::*;

    #[test]
    fn simple_str_works() {
        assert_eq!(some_str(), "four");
    }

    #[test]
    fn simple_vec_works() {
        assert_eq!(some_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn simple_devec_works() {
        assert_eq!(some_devec(), vec![1, 2]);
    }

    #[test]
    fn simple_error_works() {
        assert_eq!(
            run_error(1),
            Err(errors::CustomErrors::FirstCustomErrors(
                FirstCustomErrors::Second
            ))
        );
    }
}
