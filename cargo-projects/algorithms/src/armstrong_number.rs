fn armstrong_number(num: u32) -> i32 {
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
}
