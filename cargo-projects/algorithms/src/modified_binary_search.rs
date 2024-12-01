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

fn modified_binary_search_1(nums: &[isize], target: isize) -> isize {
    let mut left = 0;
    let mut right = nums.len() - 1;

    // While left index is less than right index
    while left < right {
        // Bitwise shift
        let mid = (left + right) >> 1;

        // Right element is less or equals than middle element
        // First test iteration "pointer" movement example
        // e.g  [[4], 5, 6, [7], 0, 1, [2]]
        //  left  ^      mid ^    right ^
        if nums[0] <= nums[mid] {
            // First element is less or equals than target
            // AND
            // target element is less or equals than mid element
            // 4 is != 0
            if nums[0] <= target && target <= nums[mid] {
                // Move right index to middle index + 1
                right = mid + 1;
            } else {
                // Move left index to middle index + 1
                // e.g. [4, 5, 6, 7, [0], 1, 2]
                //            left is ^
                left = mid + 1;
            }
        // Middle element is less than target
        // AND
        // target is less or equals to last element in the array
        } else if nums[mid] < target && target <= nums[nums.len() - 1] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if nums[left] == target {
        left as isize
    } else {
        -1
    }
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

    #[test]
    fn binary_tree_traversal_1_works() {
        let nums = [4, 5, 6, 7, 0, 1, 2];
        let target = 1;

        let actual = modified_binary_search_1(&nums, target);

        assert_eq!(actual, 5);
    }
}
