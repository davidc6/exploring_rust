use std::collections::VecDeque;

pub fn reverse_string_v1(string: &str) -> String {
    let mut reversed_str = VecDeque::new();

    string
        .chars()
        .for_each(|c| reversed_str.push_front(c.to_string()));

    let s = reversed_str.into_iter().map(|c| c.to_owned()).collect();
    s
}

pub fn reverse_string_v2(string: String) -> String {
    let mut rev_string = String::new();
    let mut chars = string;

    while let Some(char) = chars.pop() {
        rev_string.push(char);
    }

    rev_string
}

#[cfg(test)]
mod reverse_string_tests {
    use super::*;

    #[test]
    fn string_is_reversed() {
        let actual = reverse_string_v1("hello");

        assert_eq!(actual, "olleh".to_owned());
    }

    #[test]
    fn string_is_reversed_v2() {
        let actual = reverse_string_v2("hello".to_owned());

        assert_eq!(actual, "olleh".to_owned());
    }
}
