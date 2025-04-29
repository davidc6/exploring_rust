/// &[u8], &mut [u8] - reference to a portion of an array or vector (contains pointer and length).
use std::collections::HashMap;

fn count_vowels<'a>(some_collection: &'a [&str]) -> HashMap<&'a str, u8> {
    let mut chars_count = HashMap::new();
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];

    for &value in some_collection {
        for char in value.chars() {
            if vowels.contains(&char) {
                chars_count
                    .entry(value)
                    .and_modify(|val| *val += 1)
                    .or_insert(1);
            }
        }
    }

    chars_count
}

fn sum(values: &[i32]) -> i32 {
    if values.is_empty() {
        return 0;
    }

    let mut s = 0;

    for num in values {
        s += num;
    }

    // more verbose / non-optimal way
    // values.iter().copied().reduce(|acc, val| acc + val).unwrap()
    // easy way
    values.iter().sum()
}

#[cfg(test)]
mod sum_tests {
    use super::*;

    #[test]
    fn slicing_some_works() {
        let values = vec!["hello", "world", "this", "is", "me"];

        let actual = count_vowels(&values);
        let mut expected = HashMap::new();
        expected.insert("hello", 2);
        expected.insert("world", 1);
        expected.insert("this", 1);
        expected.insert("is", 1);
        expected.insert("me", 1);

        assert!(actual == expected);
    }

    #[test]
    fn empty() {
        let v = vec![];
        assert_eq!(sum(&v), 0);
    }

    #[test]
    fn one_element() {
        let v = vec![1];
        assert_eq!(sum(&v), 1);
    }

    #[test]
    fn multiple_elements() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }

    #[test]
    fn array_slice() {
        let v = [1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }
}
