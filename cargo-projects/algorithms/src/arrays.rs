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

// O(n) without division operator
// Leetcode
fn product_except_self_3(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![1; nums.len()];

    let mut prefix = 1;

    // For the length of the vector
    // Example for array [1, 2, 4, 6]
    // Prefix array [1, 1, 2, 8]
    for i in 0..nums.len() {
        result[i] = prefix;

        prefix *= nums[i];
    }

    let mut postfix = 1;

    // Example for array [1, 2, 4, 6]
    // Prefix array [48, 24, 12, 8]
    for i in (0..nums.len()).rev() {
        // multiply existing result array by the postfix value
        result[i] *= postfix;
        // then multiply postfix by nums element (starting from the last one)
        postfix *= nums[i];
    }

    result
}

fn permutations(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut counter = 1;
    let mut pointer = 0;

    let nums_len = nums.len();
    let total = if nums_len == 2 {
        2
    } else {
        nums_len * (nums_len - 1)
    };

    result.push(nums.to_owned());

    loop {
        if counter == total {
            break;
        }

        if pointer + 1 == nums_len {
            pointer = 0;
            continue;
        }

        nums.swap(pointer, pointer + 1);

        result.push(nums.to_owned());

        counter += 1;
        pointer += 1;
    }

    result
}

// Unsorted arrays find intersection
// O(n log k) k - being number of distinct elements
fn intersection(mut nums_1: Vec<i32>, mut nums_2: Vec<i32>) -> Vec<i32> {
    let mut v = vec![];

    nums_1.sort();
    nums_2.sort();

    if nums_1.len() > nums_2.len() {
        // let tmp = nums_1;
        // nums_1 = nums_2;
        // nums_2 = tmp;

        let old_nums1 = std::mem::replace(&mut nums_1, nums_2);
        nums_2 = old_nums1;
    }

    // 1 shortest
    // 2 longest
    // or equal length

    // [4, 5, 6] [1, 2, 3, 4, 5, 6, 7, 8, 9]
    //  ^     ^            ^     ^

    if nums_1.last() > nums_2.last() {
        return v;
    }

    let mut pointer_1_l = 0;
    let mut pointer_2_l = 0;

    loop {
        // println!("POINTER 1 {:?} {:?}", pointer_1_l, pointer_2_l);
        if pointer_2_l == nums_2.len() && pointer_1_l + 1 < nums_1.len() {
            pointer_1_l += 1;
        }

        if pointer_2_l == nums_2.len() && pointer_1_l == nums_1.len() {
            return v;
        }

        let n_1 = nums_1.get(pointer_1_l);
        let n_2 = nums_2.get(pointer_2_l);

        if n_1 > n_2 {
            pointer_2_l += 1;
            continue;
        }

        if n_1 < n_2 {
            return v;
        }

        if n_1 == n_2 {
            v.push(n_1.unwrap().to_owned());

            let mut pointer_1_r = pointer_1_l + 1;
            let mut pointer_2_r = pointer_2_l + 1;

            loop {
                // println!("POINTER 1 {:?} {:?}", pointer_1_r, pointer_2_r);
                if pointer_2_r == nums_2.len() && pointer_1_r + 1 < nums_1.len() {
                    pointer_1_r += 1;
                }

                if pointer_2_r == nums_2.len() && pointer_1_r == nums_1.len() {
                    return v;
                }

                let n_1 = nums_1.get(pointer_1_r);
                let n_2 = nums_2.get(pointer_2_r);

                if n_1 < n_2 {
                    pointer_2_r += 1;
                    continue;
                }

                if n_1 > n_2 {
                    return v;
                }

                if n_1 == n_2 {
                    v.push(n_1.unwrap().to_owned());

                    pointer_1_r += 1;
                    pointer_2_r += 1;

                    continue;
                }
            }
        }
    }

    v
}

fn sum_of_three_integers(nums: Vec<u32>, num: u32) -> bool {
    let mut start_index: usize = 0;
    let mut mid_index: usize = 1;
    let mut end_index: usize = 2;

    loop {
        let sum = nums.get(start_index).unwrap()
            + nums.get(mid_index).unwrap()
            + nums.get(end_index).unwrap();

        if sum == num {
            return true;
        }

        if start_index + 2 == nums.len() - 1 {
            return false;
        }

        if mid_index + 1 == nums.len() - 1 {
            start_index += 1;
            mid_index = start_index + 1;
            end_index = mid_index + 1;
            continue;
        }

        if end_index == nums.len() - 1 {
            mid_index += 1;
            end_index = mid_index + 1;
            continue;
        }

        end_index += 1;
    }
}

fn removing_item(arr: &mut [usize], position: usize) {
    if arr.len() == 1 {
        return;
    }

    if position == arr.len() - 1 {
        arr[position] = arr[position - 1];
        return;
    }

    for x in 0..arr.len() {
        if x == position {
            let mut end = arr.len() - 1;
            let mut start = end - 1;
            let mut prev = None;

            loop {
                if end == position {
                    break;
                }

                if let Some(p) = prev {
                    let t = arr[start];
                    arr[start] = p;
                    prev = Some(t);
                } else {
                    prev = Some(arr[start]);
                    arr[start] = arr[end]
                }

                if start == 0 {
                    break;
                }

                start -= 1;
                end -= 1;
                continue;
            }
        }
    }
}

#[cfg(test)]
mod arrays_tests {
    use super::{product_except_self, sum_of_three_integers};
    use crate::arrays::{
        intersection, permutations, product_except_self_2, product_except_self_3, removing_item,
    };

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

    #[test]
    fn product_except_self_works_5() {
        let nums = vec![-1, 0, 1, 2, 3];
        let actual = product_except_self_3(nums);
        let expected = [0, -6, 0, 0, 0];

        assert_eq!(actual, expected);
    }

    #[test]
    fn product_except_self_works_6() {
        let nums = vec![1, 2, 4, 6];
        let actual = product_except_self_3(nums);
        let expected = [48, 24, 12, 8];

        assert_eq!(actual, expected);
    }

    #[test]
    fn permutations_works() {
        let nums = vec![1, 2, 3];
        let expected = vec![
            vec![1, 2, 3],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 2, 1],
            vec![3, 1, 2],
            vec![1, 3, 2],
        ];
        let actual = permutations(nums);

        assert_eq!(actual, expected);
    }

    #[test]
    fn permutations_works_2() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![
            vec![1, 2, 3, 4],
            vec![2, 1, 3, 4],
            vec![2, 3, 1, 4],
            vec![2, 3, 4, 1],
            vec![3, 2, 4, 1],
            vec![3, 4, 2, 1],
            vec![3, 4, 1, 2],
            vec![4, 3, 1, 2],
            vec![4, 1, 3, 2],
            vec![4, 1, 2, 3],
            vec![1, 4, 2, 3],
            vec![1, 2, 4, 3],
        ];
        let actual = permutations(nums);

        assert_eq!(actual, expected);
    }

    #[test]
    fn permutations_works_3() {
        let nums = vec![0, 1];
        let expected = vec![vec![0, 1], vec![1, 0]];
        let actual = permutations(nums);

        assert_eq!(actual, expected);
    }

    #[test]
    fn intersection_works() {
        let nums = vec![3, 4, 5];
        let nums_2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let expected = vec![3, 4, 5];
        let actual = intersection(nums, nums_2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn sum_of_three_ints_works() {
        let nums = vec![3, 7, 1, 2, 8, 4, 5];
        let number = 21;

        let actual = sum_of_three_integers(nums, number);
        assert!(!actual);

        let nums = vec![3, 7, 1, 2, 9, 4, 5];
        let number = 21;

        let actual = sum_of_three_integers(nums, number);
        assert!(actual);
    }

    #[test]
    fn removing_item_at_position_2_from_array() {
        let mut arr = [1, 2, 3, 4, 5];
        let position = 2;

        let expected = [1, 2, 4, 5, 5];
        removing_item(&mut arr, position);

        assert_eq!(arr, expected);
    }

    #[test]
    fn removing_item_at_position_start_from_array() {
        let mut arr = [1, 2, 3, 4, 5];
        let position = 0;

        let expected = [2, 3, 4, 5, 5];
        removing_item(&mut arr, position);

        assert_eq!(arr, expected);
    }

    #[test]
    fn removing_item_at_the_end_from_array() {
        let mut arr = [1, 2, 3, 4, 5];
        let position = 4;

        let expected = [1, 2, 3, 4, 4];
        removing_item(&mut arr, position);

        assert_eq!(arr, expected);
    }
}
