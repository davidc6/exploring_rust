fn sum(values: &[i32]) -> i32 {
    if values.is_empty() {
        return 0;
    }

    let mut s = 0;

    for num in values {
        s += num;
    }

    // more verbose / non-optimal way
    // values.iter().copied().reduce(|acc, val| acc + val).unwrap()
    // easy way
    values.iter().sum()
}

#[cfg(test)]
mod sum_tests {
    use super::*;

    #[test]
    fn empty() {
        let v = vec![];
        assert_eq!(sum(&v), 0);
    }

    #[test]
    fn one_element() {
        let v = vec![1];
        assert_eq!(sum(&v), 1);
    }

    #[test]
    fn multiple_elements() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }

    #[test]
    fn array_slice() {
        let v = [1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }
}
