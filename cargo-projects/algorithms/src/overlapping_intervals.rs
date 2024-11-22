type Merge = Vec<Vec<i32>>;

fn merge_1(input: Merge) -> Merge {
    input
}

#[cfg(test)]
mod intervals_test {
    use super::merge_1;

    #[test]
    fn merge_1_works() {
        let input = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let output = merge_1(input.clone());
        assert_eq!(input, output);
    }
}
