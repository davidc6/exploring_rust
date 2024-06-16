use std::{collections::HashMap, fmt::Debug, hash::Hash};

// T is bound by Debug, Hash and Eq
// Debug allows to debug by printing using println!() and {:?} specifier
// Hash allows values to be hashed using the instance of Hasher
// Eq provides full equivalence relationship comparison
pub fn is_permutation<T: Debug + Eq + Hash>(vec_one: Vec<T>, vec_two: Vec<T>) -> bool {
    if vec_one.len() != vec_two.len() {
        return false;
    }

    let mut char_count = HashMap::new();

    // Create a consuming Iterator from vec_one
    // and build a HashMap out of it by inserting and modifying
    vec_one.into_iter().for_each(|character| {
        char_count
            .entry(character)
            .and_modify(|val| *val += 1)
            .or_insert(1);
    });

    for character in vec_two {
        if let Some(val) = char_count.get_mut(&character) {
            if val == &mut 0 {
                return false;
            }

            *val -= 1;
            continue;
        }

        return false;
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

    #[test]
    fn vecs_are_not_permutations() {
        let vec_one = vec!["a", "b", "c"];
        let vec_two = vec!["a", "b", "d"];

        let actual = is_permutation(vec_one, vec_two);

        assert!(!actual);
    }

    #[test]
    fn vecs_are_not_permutations_2() {
        let vec_one = vec!["a", "b", "c"];
        let vec_two = vec!["b", "b", "a"];

        let actual = is_permutation(vec_one, vec_two);

        assert!(!actual);
    }

    #[test]
    fn vec_permutation() {
        let vec_one = vec!["a", "b", "c"];
        let vec_two = vec!["c", "b", "a"];

        let actual = is_permutation(vec_one, vec_two);

        assert!(actual);
    }
}
