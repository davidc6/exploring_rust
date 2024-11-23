//! # Top K Elements
//!  
//! Sorting vectors/arrays takes O(n log n).
//! Using min or max heap takes O(n log k), where k
//!
//! k largest - min-heap
//! k smallest - max-heap
//!

use std::collections::BinaryHeap;

fn top_1(input: Vec<i32>, k: u32) -> Vec<i32> {
    let mut b_h = BinaryHeap::from(input);

    let mut output = vec![];

    for _ in 0..k {
        output.push(b_h.pop().unwrap());
    }

    output
}

#[cfg(test)]
mod top_k_elements_tests {
    use super::*;

    #[test]
    fn top_1_works() {
        let v = vec![1, 5, 11, 9, 7, 2];
        let result = top_1(v, 3);

        assert_eq!(result, vec![11, 9, 7]);
    }
}
