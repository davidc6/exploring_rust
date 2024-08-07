pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = Vec::with_capacity(20);
    let mut number = num;

    while number > 0 {
        let current_digit = number % 10;
        number /= 10;
        digits.push(current_digit);
    }

    let digits_count = digits.len() as u32;
    let sum = digits.into_iter().fold(0, |acc: u32, v| {
        if let Some(result) = acc.checked_add(v.pow(digits_count)) {
            result
        } else {
            acc
        }
    });

    if sum == num {
        return true;
    }

    false
}

#[cfg(test)]
mod armstrong_number_tests {
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
