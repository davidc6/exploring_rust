fn iter_1(arr: &[i32]) -> Vec<i32> {
    let a = arr.to_owned();
    let a = a.iter().filter(|&a| *a > 10).copied().collect();
    a
}

#[cfg(test)]
mod iterators_tests {
    use super::*;

    #[test]
    fn iter_1_works() {
        let ints = [20, 13, 7, 6, 8];

        let actual = iter_1(&ints);
        let expected = vec![20, 13];

        assert_eq!(actual, expected);
    }
}
