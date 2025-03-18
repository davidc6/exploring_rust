pub fn factorial(num: usize) -> usize {
    run_factorial(num, 1)
}

fn run_factorial(current_num: usize, mut total: usize) -> usize {
    if current_num == 0 {
        return total;
    }

    total *= current_num;

    run_factorial(current_num - 1, total)
}

#[cfg(test)]
mod tests {
    use super::factorial;

    #[test]
    fn factorial_works_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_works_1() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn factorial_works_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_works() {
        let actual = factorial(5);
        assert_eq!(actual, 120);
    }

    #[test]
    fn factorial_works_large_number() {
        let actual = factorial(10);
        assert_eq!(actual, 3628800);
    }
}
