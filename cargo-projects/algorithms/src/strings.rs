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

fn encode_decode_strings(strs: &mut [&str]) -> String {
    let mut result = String::new();

    for value in strs {
        let length = value.len();
        let formatted_str = format!("{length}${value}");

        result.push_str(&formatted_str);
    }

    result
}

fn decode_strings(str: &str) -> Vec<String> {
    let mut count = "".to_string();
    let mut count_i = 0;
    let mut strs = vec![];
    let mut cur_str = "".to_string();
    let mut on_str = false;

    for char in str.chars() {
        if on_str && count_i > 0 {
            cur_str.push_str(&char.to_string());
            count_i -= 1;
            continue;
        }

        if on_str && count_i == 0 {
            strs.push(cur_str.to_owned());
            on_str = false;
            count_i = 0;
            count.clear();
            cur_str.clear();
        }

        if char == '$' {
            count_i = count.parse().unwrap();
            on_str = true;
            continue;
        } else {
            count.push_str(&char.to_string());
        }
    }

    strs.push(cur_str.to_owned());

    strs
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

    #[test]
    fn encode_strings_works() {
        let mut strs = ["hello", "hi", "hey"];
        let actual = encode_decode_strings(&mut strs);
        assert_eq!(actual, "5$hello2$hi3$hey".to_owned());
    }

    #[test]
    fn decode_strings_works() {
        let str = "5$hello2$hi3$hey";
        let expected = vec!["hello".to_string(), "hi".to_string(), "hey".to_string()];
        let actual = decode_strings(str);

        assert_eq!(expected, actual);
    }
}
