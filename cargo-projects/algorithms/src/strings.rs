fn remove_single_vowels(input: &str) -> String {
    let vowels = ["a", "e", "i", "o", "u"];

    let mut processed_input = vec![];

    let mut prev_char = "";

    for (index, value) in input.split("").enumerate() {
        if index == 0 {
            processed_input.push(value);
            continue;
        }

        if vowels.contains(&value) && vowels.contains(&prev_char) {
            processed_input.push(prev_char);
            processed_input.push(value);
            prev_char = "";
            continue;
        }

        if !vowels.contains(&value) {
            processed_input.push(value);
        }

        prev_char = value;
    }

    processed_input.join::<&str>("")
}

#[cfg(test)]
mod string_tests {
    use super::*;

    #[test]
    fn remove_single_vowels_works() {
        let string = "car";

        let actual = remove_single_vowels(string);

        assert_eq!(actual, "cr".to_owned());
    }

    #[test]
    fn remove_single_vowels_works_with_two_vowels() {
        let string = "caar";

        let actual = remove_single_vowels(string);

        assert_eq!(actual, "caar".to_owned());
    }

    #[test]
    fn remove_single_vowels_works_with_three_vowels() {
        let string = "caaar";

        let actual = remove_single_vowels(string);

        assert_eq!(actual, "caar".to_owned());
    }
}
