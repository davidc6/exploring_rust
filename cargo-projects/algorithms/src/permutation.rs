use std::{collections::HashMap, hash::Hash};

fn is_permutation<T: Eq + Hash>(vec_one: Vec<T>, vec_two: Vec<T>) -> bool {
    if vec_one.len() != vec_two.len() {
        return false;
    }

    let mut chars_map = HashMap::new();

    for character in vec_one {
        *chars_map.entry(character).or_insert(1) += 1;
    }

    true
}

#[cfg(test)]
mod permutation_tests {
    use super::*;

    #[test]
    fn vecs_are_not_same_length() {
        let vec_one = vec!["a", "b", "c"];
        let vec_two = vec!["a", "b", "c", "d"];

        let actual = is_permutation(vec_one, vec_two);

        assert!(!actual);
    }
}
