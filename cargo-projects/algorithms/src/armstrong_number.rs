pub fn is_armstrong_number(num: u32) -> bool {
    let mut base = 10;
    let mut digit_count = 1;

    while num / base != 0 {
        let r = base.checked_mul(10);

        if r.is_none() {
            break;
        }

        base = r.unwrap();
        digit_count += 1;
    }

    base = 1;
    let mut sum = 0;

    for i in 0..digit_count {
        base *= 10;

        let digit = ((num as f64 / base as f64).fract() * 10.0) as u32;

        println!(
            "NUM {:?} {} {:?}",
            i,
            // ((num as f64 / base as f64).fract()),
            // (num as f64 * 100.0).round() / 100.0,
            num as f64 / 10.0,
            // Decimal::from_f64_retain(n).unwrap();
            digit
        );

        sum += digit.pow(digit_count as u32);
    }

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
