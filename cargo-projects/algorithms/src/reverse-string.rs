use std::collections::VecDeque;

pub fn reverse_str_v0(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn reverse_string_v1(string: &str) -> String {
    let mut reversed_str = VecDeque::new();

    string
        .chars()
        .for_each(|c| reversed_str.push_front(c.to_string()));

    reversed_str.into_iter().map(|c| c.to_owned()).collect()
}

pub fn reverse_string_v2(string: &str) -> String {
    let mut rev_string = String::new();
    let mut chars = String::from(string);

    while let Some(char) = chars.pop() {
        rev_string.push(char);
    }

    rev_string
}

pub fn reverse_string_v3(string: &mut String) -> String {
    let mut bytes = std::mem::take(string).into_bytes();

    let mut last = bytes.len();
    let mut first = 0;

    while last > first {
        last -= 1;

        bytes.swap(first, last);

        first += 1;
    }

    String::from_utf8(bytes).expect("conversion failed")
}

#[cfg(test)]
mod reverse_string_tests {
    use super::*;

    #[test]
    fn string_is_reversed_v0() {
        let actual = reverse_str_v0("hello");

        assert_eq!(actual, "olleh".to_owned());
    }

    #[test]
    fn string_is_reversed() {
        let actual = reverse_string_v1("hello");

        assert_eq!(actual, "olleh".to_owned());
    }

    #[test]
    fn string_is_reversed_v2() {
        let actual = reverse_string_v2("hello");

        assert_eq!(actual, "olleh".to_owned());
    }

    #[test]
    fn string_is_reversed_v3() {
        let actual = reverse_string_v3(&mut "hello".to_owned());

        assert_eq!(actual, "olleh".to_owned());
    }
}
