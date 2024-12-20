//! # Top K Elements
//!  
//! Sorting vectors/arrays takes O(n log n).
//! Using min or max heap takes O(n log k), where k
//!
//! k largest  - min-heap
//! k smallest - max-heap

use std::collections::{BinaryHeap, HashMap};

fn top_1(input: Vec<i32>, k: u32) -> Vec<i32> {
    let mut b_h = BinaryHeap::from(input);

    let mut output = vec![];

    for _ in 0..k {
        output.push(b_h.pop().unwrap());
    }

    output
}

fn top_2(input: Vec<i32>, k: u32) -> Vec<i32> {
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

#[derive(PartialEq, Clone, Eq, Ord, PartialOrd, Debug)]
struct Count(i32, i32);

#[derive(PartialEq, Eq, Clone, Debug)]
struct MostFrequentFirst(Count);

// PartialOrd - enables us to specify the behaviour of the ordered comparison operators (<, >, <=, >=)
impl PartialOrd for MostFrequentFirst {
    // The only method that requires implementation
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Calling out to Ord cmp() method
        Some(self.cmp(other))
    }
}

impl Ord for MostFrequentFirst {
    // The only method that requires implementation
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // self.0 .0 is Count and first value
        // we compare by frequency
        self.0 .0.cmp(&other.0 .0)
    }
}

// Leetcode 347, Top K Frequent Elements
// O (n * log( k ))
fn top_3(input: Vec<i32>, k: i32) -> Vec<i32> {
    let frequencies = input.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val)
            .and_modify(|frequency| *frequency += 1)
            .or_insert(1);
        map
    });

    let mut heap = BinaryHeap::new();

    for (key, freq) in frequencies {
        // The trick here is to make integers signed (negative) so that they do not get popped()
        // -7 -3
        if heap.len() < k as usize {
            heap.push(MostFrequentFirst(Count(-freq, key)));
        } else {
            // We want to make sure that no more than k elements are kept on the heap,
            // hence we push and pop here
            heap.push(MostFrequentFirst(Count(-freq, key)));
            // removes the greatest item which in our case would be least negative number
            // -7 -3 1 , 1 - gets popped
            heap.pop();
        }
    }

    let sorted_vec = heap.into_sorted_vec();
    Vec::from(&sorted_vec[(sorted_vec.len() - k as usize)..sorted_vec.len()])
        .iter()
        .map(|val| val.0 .1)
        .collect()
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

    #[test]
    fn top_3_works() {
        let v = vec![1, 1, 1, 2, 2, 3, 5, 5, 5, 5, 5];
        let c = top_3(v, 2);

        assert_eq!(c, vec![5, 1]);
    }
}
