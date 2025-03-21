fn simple_binary_search(nums: &[isize], target: isize) -> Option<usize> {
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
            return Some(mid);
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

    None
}

// Leetcode 33, Search in Rotated Sorted ArrayA
// Ref https://leetcode.com/problems/search-in-rotated-sorted-array/description/
fn modified_binary_search_1(nums: &[isize], target: isize) -> Option<usize> {
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
        Some(left)
    } else {
        None
    }
}

// Leetcode 189, Rotate Array
// https://leetcode.com/problems/rotate-array/description/
fn rotate_array_right(nums: &mut [isize], times: usize) -> &mut [isize] {
    // e.g. total 3 times
    // [1, 2, 3, 4]
    // [4, 1, 2, 3]
    // [3, 4, 1, 2]
    // [2, 3, 4, 1]
    for _ in 0..times {
        let last = nums[nums.len() - 1];

        for i in (0..nums.len()).rev() {
            if i == 0 {
                break;
            }

            nums[i] = nums[i - 1];
        }

        nums[0] = last;
    }

    nums
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
            assert_eq!(actual, Some(index));
        }
    }

    #[test]
    fn binary_tree_traversal_1_works() {
        let nums = [4, 5, 6, 7, 0, 1, 2];
        let target = 1;

        let actual = modified_binary_search_1(&nums, target);

        assert_eq!(actual, Some(5));
    }

    #[test]
    fn rotate_array_right_works() {
        let mut nums = [1, 2, 3, 4];

        let actual = rotate_array_right(&mut nums, 3);

        assert_eq!(actual, &mut [2, 3, 4, 1]);
    }

    #[test]
    fn rotate_array_right_works_2() {
        let mut nums = [-1, -100, 3, 99];

        let actual = rotate_array_right(&mut nums, 2);

        assert_eq!(actual, &mut [3, 99, -1, -100]);
    }
}
