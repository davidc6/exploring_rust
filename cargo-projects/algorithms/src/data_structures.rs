use std::collections::HashMap;
use std::collections::LinkedList;
use std::collections::VecDeque;

fn vector() {
    // Vector
    let mut v = vec![];

    // insert
    v.push(1);
    // remove and return last element
    v.pop();
    // insert 3 more vals
    v.push(10);
    v.push(50);
    v.push(100);
    // get element at index 1 i.e. 50
    let _ = v.get(1); // Some(50)

    // remove element at index 1 i.e. 50
    v.remove(1);

    // binary search, only if sorted

    // filter aka retain, [100]
    v.retain(|&val| val > 50);

    // Concat vectors
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    // new vector is created
    let v3 = [v1, v2];

    // We don't need to keep v1 or v2 intact
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![4, 5, 6];
    // v2 elements appended to v1, v2 emptied
    v1.append(&mut v2);

    // Vec that we extend to , stays intact
    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    // v2 is intact, v1 extended
    v1.extend(&v2);

    // Positioning
    let v1 = [1, 2, 3];

    let val = 1;
    let test = v1.iter().position(|&n| n == val);
    println!("Position of value {:?}: {:?}", val, test);

    let index = 0;
    let test = v1.get(index);
    println!("Value at index {:?}: {:?}", index, test);
}

pub fn double_ended_queue() {
    // VecDeque - double-ended queue implemented with a growable ring buffer
    let mut vd = VecDeque::new();

    // [1, 2, 3]
    vd.push_back(1);
    vd.push_back(2);
    vd.push_back(3);
    // related - pop_back()

    // [4, 5, 6, 1, 2, 3]
    vd.push_front(6);
    vd.push_front(5);
    vd.push_front(4);
    // related - pop_front()

    // check if a value exists in the vector
    vd.contains(&6);
    // check if vector is empty
    vd.is_empty();

    println!("{:?}", vd);
}

// HashMap uses SIMD (Single Instruction Multiple Data) lookup
pub fn hash_map() -> Option<i32> {
    let mut h = HashMap::new();

    let s = "one";
    let val = h.insert(s, 1);

    if let Some(val) = val {
        println!("Overwritting {:?}", val);
    } else {
        println!("Writing initial value {:?}", s);
    }

    let s2 = "two";

    let val = h.entry(s2).or_insert(2);
    *val += 1;

    h.get(s2).copied()
}

#[cfg(test)]
mod hash_map_tests {
    use super::*;

    #[test]
    fn hash_map_works() {
        assert_eq!(hash_map(), Some(3));
    }
}
