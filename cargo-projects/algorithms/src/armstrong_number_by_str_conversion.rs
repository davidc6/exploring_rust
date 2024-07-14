pub fn is_armstrong_number(num: u32) -> bool {
    let num_as_str = num.to_string();
    let mut sum: usize = 0;

    for character in num_as_str.chars() {
        let n = character.to_digit(10).unwrap() as usize;
        sum += n.pow(num_as_str.len() as u32);

        if sum > num as usize {
            return false;
        }
    }

    if sum == num as usize {
        return true;
    }

    false
}

#[cfg(test)]
mod is_armstrong_number_digit_tests {
    use super::*;

    #[test]
    fn zero_is_an_armstrong_number() {
        assert!(is_armstrong_number(0))
    }

    #[test]
    fn single_digit_numbers_are_armstrong_numbers() {
        assert!(is_armstrong_number(5))
    }

    #[test]
    fn there_are_no_2_digit_armstrong_numbers() {
        assert!(!is_armstrong_number(10))
    }

    #[test]
    fn three_digit_armstrong_number() {
        assert!(is_armstrong_number(153))
    }

    #[test]
    fn three_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(100))
    }

    #[test]
    fn four_digit_armstrong_number() {
        assert!(is_armstrong_number(9474))
    }

    #[test]
    fn four_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(9475))
    }

    #[test]
    fn seven_digit_armstrong_number() {
        assert!(is_armstrong_number(9_926_315))
    }

    #[test]
    fn seven_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(9_926_316))
    }

    #[test]
    fn nine_digit_armstrong_number() {
        assert!(is_armstrong_number(912_985_153));
    }

    #[test]
    fn nine_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(999_999_999));
    }

    #[test]
    fn ten_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(3_999_999_999));
    }

    #[test]
    fn properly_handles_overflow() {
        assert!(!is_armstrong_number(4_106_098_957));
    }
}
