// O(n^2)
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    for (index, _) in nums.iter().enumerate() {
        let mut total = 1;

        for (index_2, val_2) in nums.iter().enumerate() {
            if index == index_2 {
                continue;
            }
            total *= val_2;
        }

        result.push(total);
    }

    result
}

// O(n)
fn product_except_self_2(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    let mut sum = 1;
    let mut is_zero = false;

    for val in nums.iter() {
        if *val == 0 {
            is_zero = true;
            continue;
        }
        sum *= val;
    }

    for val in nums.into_iter() {
        let a = if val == 0 {
            sum
        } else if is_zero {
            0
        } else {
            sum / val
        };

        result.push(a);
    }

    result
}

#[cfg(test)]
mod arrays_tests {
    use crate::arrays::product_except_self_2;

    use super::product_except_self;

    #[test]
    fn product_except_self_works() {
        let nums = vec![1, 2, 4, 6];
        let actual = product_except_self(nums);
        let expected = [48, 24, 12, 8];

        assert_eq!(actual, expected);
    }

    #[test]
    fn product_except_self_works_2() {
        let nums = vec![-1, 0, 1, 2, 3];
        let actual = product_except_self(nums);
        let expected = [0, -6, 0, 0, 0];

        assert_eq!(actual, expected);
    }

    #[test]
    fn product_except_self_works_3() {
        let nums = vec![1, 2, 4, 6];
        let actual = product_except_self_2(nums);
        let expected = [48, 24, 12, 8];

        assert_eq!(actual, expected);
    }

    #[test]
    fn product_except_self_works_4() {
        let nums = vec![-1, 0, 1, 2, 3];
        let actual = product_except_self_2(nums);
        let expected = [0, -6, 0, 0, 0];

        assert_eq!(actual, expected);
    }
}
