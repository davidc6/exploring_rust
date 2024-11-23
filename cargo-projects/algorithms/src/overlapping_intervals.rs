type Merge = Vec<Vec<i32>>;

fn merge_1(mut input: Merge) -> Merge {
    input.sort_by(|a, b| a.get(1).unwrap().cmp(b.get(1).unwrap()));

    let mut output = vec![];

    for interval in input {
        if output.is_empty() {
            output.push(interval);
            continue;
        }

        let last = output.last_mut().unwrap();

        if !(interval.first().unwrap()..interval.get(1).unwrap()).contains(&last.get(1).unwrap()) {
            println!("No contain {:?} {:?}", interval, last.get(1).unwrap());
            output.push(interval);
            continue;
        }

        // contains interval
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
}
