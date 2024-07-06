pub fn armstrong_number(num: u32) -> bool {
    let mut current_ten = 10;
    let mut count = 1;

    while num / current_ten != 1 {
        current_ten *= 10;
        count += 1;
    }

    count += 1;

    current_ten = 1;
    let mut sum = 0;

    for _ in 0..count {
        current_ten *= 10;

        let digit = ((num as f32 / current_ten as f32).fract() * 10.0) as u32;

        sum += digit.pow(count as u32);
    }

    if sum == num {
        return true;
    }

    false
}

pub fn armstrong_number_v2(num: u32) -> bool {
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
mod armstrong_number_tests {
    use super::*;

    #[test]
    fn armstrong_number_runs_with_single_digit() {
        let actual = armstrong_number(153);

        assert!(actual);
    }

    #[test]
    fn armstrong_number_runs_with_two_digits() {
        let actual = armstrong_number(10);

        assert!(!actual);
    }

    // #[test]
    // fn armstrong_number_runs_with_three_digits() {
    //     let actual = armstrong_number(145);

    //     assert_eq!(actual, 3);
    // }

    #[test]
    fn armstrong_number_v2_returns_true() {
        let actual = armstrong_number_v2(153);
        assert!(actual);
    }

    #[test]
    fn armstrong_number_v2_returns_false() {
        let actual = armstrong_number_v2(10);
        assert!(!actual);
    }
}
