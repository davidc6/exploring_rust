//! # Top K Elements
//!  
//! Sorting vectors/arrays takes O(n log n).
//! Using min or max heap takes O(n log k), where k
//!
//! k largest  - min-heap
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

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn top_2(mut input: Vec<i32>, k: u32) -> Vec<i32> {
    let mut m_h = vec![];

    for val in input {
        m_h.push(val);
        if m_h.len() == 1 {
            continue;
        }
        let l = m_h.len() - 1;
        top_2_r(&mut m_h, l);
    }

    m_h
}

fn get_parent(index: usize) -> usize {
    index / 2
}

fn top_2_r(min_heap: &mut [i32], index: usize) {
    let parent_index = get_parent(index);
    let min_heap_length = index;

    if min_heap[parent_index] > min_heap[min_heap_length] {
        min_heap.swap(parent_index, min_heap_length);

        top_2_r(min_heap, parent_index);
    }
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

    #[test]
    fn top_2_works() {
        let v = vec![10, 8, 15, 3];
        let output = top_2(v, 1);

        assert_eq!(output, vec![3, 8, 10, 15]);
    }
}
