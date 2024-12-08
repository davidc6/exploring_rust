// O(n2) - two nested loops
fn find_next_greater_elem(nums: &[i32], num: i32) -> Vec<i32> {
    let mut vec = vec![];
    // 4, 5, 2, 25

    let mut has_added = false;

    // queue - first in first out
    for (index, orig_val) in nums.iter().enumerate() {
        for val in &nums[index..nums.len()] {
            if val > orig_val {
                vec.push(*val);
                has_added = true;
                break;
            }
        }

        if !has_added {
            vec.push(-1);
        }
    }

    vec.push(-1);

    vec
}

// Stack based
// Variation of Leetcode 496, Next Greater Element I - https://leetcode.com/problems/next-greater-element-i/description/
fn find_next_greater_elem_stack(nums: &[i32], num: i32) -> Vec<i32> {
    let mut result = vec![];
    let mut stack = vec![];

    // Stack initially is empty so push onto it
    stack.push(nums[0]);

    // Iterate over the array slice
    for val in &nums[1..nums.len()] {
        while !stack.is_empty() && stack.last() < Some(val) {
            result.push(*val);
            stack.pop();
        }

        stack.push(*val);
    }

    result.push(-1);

    result
}

#[cfg(test)]
mod stack_tests {
    use crate::stack::find_next_greater_elem_stack;

    use super::find_next_greater_elem;

    #[test]
    fn find_next_greater_elem_works() {
        let a = [4, 5, 2, 25];

        let actual = find_next_greater_elem(&a, 5);

        assert_eq!(actual, vec![5, 25, 25, -1]);
    }

    #[test]
    fn find_next_greater_elem_stack_works() {
        let a = [4, 5, 2, 25];

        let actual = find_next_greater_elem_stack(&a, 5);

        assert_eq!(actual, vec![5, 25, 25, -1]);
    }
}
