type Merge = Vec<Vec<i32>>;

fn merge_1(mut input: Merge) -> Merge {
    // First, sort the vector by end value
    input.sort_by(|a, b| a.get(1).unwrap().cmp(b.get(1).unwrap()));

    let mut output = vec![];

    // Iterate over the vector
    for interval in input {
        // Push initial vector
        if output.is_empty() {
            output.push(interval);
            continue;
        }

        // Get last vector in the new output vector
        let last = output.last_mut().unwrap();

        // Check if end is NOT in range of the current iteration vector,
        // if it is not then push as new vector into the output vector
        if !(interval.first().unwrap()..interval.get(1).unwrap()).contains(&last.get(1).unwrap()) {
            output.push(interval);
            continue;
        }

        // End is in range, so determine if we need to swap the start value
        let i = *interval.first().unwrap();
        last[0] = if i < last[0] { i } else { last[0] };
        last[1] = *interval.get(1).unwrap();
    }

    output
}

#[cfg(test)]
mod intervals_test {
    use super::merge_1;

    #[test]
    fn merge_1_works() {
        let input = vec![vec![2, 6], vec![1, 3], vec![15, 18], vec![8, 10]];
        let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        let output = merge_1(input);
        assert_eq!(expected, output);
    }

    #[test]
    fn merge_2_works() {
        let input = vec![vec![1, 4], vec![4, 5]];
        let expected = vec![vec![1, 5]];
        let output = merge_1(input);
        assert_eq!(expected, output);
    }
}
