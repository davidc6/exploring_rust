fn iter_1(arr: &[i32]) -> Vec<i32> {
    let vec_owned = arr.to_owned();
    vec_owned.iter().filter(|&a| *a > 10).copied().collect()
}

fn iter_2(arr: &mut [i32]) -> &mut [i32] {
    for element in arr.iter_mut() {
        if *element > 10 {
            *element *= 10;
        }
    }

    arr
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

    #[test]
    fn iter_2_works() {
        let mut ints = [20, 13, 7, 6, 8];

        let actual = iter_2(&mut ints);
        let expected = vec![200, 130, 7, 6, 8];

        assert_eq!(actual, expected);
    }
}
