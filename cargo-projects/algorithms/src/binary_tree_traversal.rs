fn simple_binary_search(nums: &[isize], target: isize) -> usize {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        // Bitwise shift move bits to the right.
        // For example, 8 >> 1 becomes 4:
        // 8 is 00001000, 4 is 00000100
        //          ^               ^
        // you can see that the bit gets shifted by 1 to the right.
        // In the case of odd numbers,
        // it's essential "floor" i.e. lowest number
        let mid = (left + right) >> 1;

        if nums[mid] == target {
            return mid;
        }

        if nums[mid] < target {
            left = mid + 1;
            continue;
        }

        if nums[mid] > target {
            right = mid - 1;
            continue;
        }
    }

    0
}

#[cfg(test)]
mod binary_tree_traversal_tests {
    use super::*;

    #[test]
    fn simple_binary_search_works() {
        let nums = [0, 1, 2, 4, 5, 6, 7];

        let mut expected = [(-1, 0); 7];
        for (index, value) in nums.iter().enumerate() {
            let i = &mut expected;
            i[index] = (*value, index);
        }

        for expect in expected {
            let (val, index) = expect;

            let actual = simple_binary_search(&nums, val);
            assert_eq!(actual, index);
        }
    }

    fn binary_tree_traversal_1_works() {
        // let nums = [4, 5, 6, 7, 0, 1, 2];
    }
}
