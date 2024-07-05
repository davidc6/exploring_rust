pub fn armstrong_number(num: u32) -> i32 {
    let mut current_ten = 10;
    let mut count = 1;

    if num / current_ten == 0 {
        return 1;
    }

    while num / current_ten != 1 {
        println!("{:?}", current_ten);
        current_ten *= 10;
        count += 1;
    }

    count += 1;

    count
}

pub fn armstrong_number_v2(num: u32) -> bool {
    let num_as_str = num.to_string();
    let len_of_str = num_as_str.len();
    let mut sum = 0;

    for character in num_as_str.chars() {
        let n = character.to_digit(10).unwrap() as usize;
        sum += n.pow(len_of_str as u32);

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
        let actual = armstrong_number(1);

        assert_eq!(actual, 1);
    }

    #[test]
    fn armstrong_number_runs_with_two_digits() {
        let actual = armstrong_number(14);

        assert_eq!(actual, 2);
    }

    #[test]
    fn armstrong_number_runs_with_three_digits() {
        let actual = armstrong_number(145);

        assert_eq!(actual, 3);
    }

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
