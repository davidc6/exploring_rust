use std::collections::VecDeque;

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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn run_all() {
    let longest_str = some_str();
    println!("{:?}", longest_str);

    let a = some_vec();

    println!("{:?}", a);

    // for val in a {
    //     println!("{:?}", val);
    // }

    let b = some_devec().pop_front();

    // for val in b.iter() {
    //     println!("{:?}", val);
    // }

    println!("{:?}", b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);

        run_all();
    }
}
