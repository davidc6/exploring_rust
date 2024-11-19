use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::mem;

// Vector - is a collection homogenous elements
// Heterogeneous vector type can be made by using trait objects or enums (if all variants are known)
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

#[derive(Debug)]
struct DeviceStatus {
    id: u16,
    last_logged_in: u32,
}

impl DeviceStatus {
    pub fn new(id: u16, last_logged_in: u32) -> Self {
        DeviceStatus { id, last_logged_in }
    }
}

fn vector_struct_sort() -> Vec<DeviceStatus> {
    let mut devices = vec![
        DeviceStatus::new(1, 1731844615),
        DeviceStatus::new(2, 1731844615),
        DeviceStatus::new(3, 1729072551),
    ];

    // Sort by last_logged_in field in the ascending order (earliest timestamp first)
    devices.sort_by(|a, b| a.last_logged_in.cmp(&b.last_logged_in));

    devices
}

trait Accelerate {
    fn accelerate(&mut self);
}

trait Speed {
    fn speed(&self) -> u8;
}

trait Vehicle: Accelerate + Speed {}
// Blanket implementation (an implementation of a trait for all types or for all types that match
// some condition) i.e. if a type implements Accelerate and Speed,
// then it also implements Vehicle
impl<T: Accelerate + Speed> Vehicle for T {}

// Heterogenous vector
// A vector that contains different types

#[derive(Debug)]
struct Airship {
    speed: u8,
}

impl Accelerate for Airship {
    fn accelerate(&mut self) {
        self.speed += 10;
    }
}

impl Speed for Airship {
    fn speed(&self) -> u8 {
        self.speed
    }
}

#[derive(Debug)]
struct Bike {
    speed: u8,
}

impl Accelerate for Bike {
    fn accelerate(&mut self) {
        self.speed += 5;
    }
}

impl Speed for Bike {
    fn speed(&self) -> u8 {
        self.speed
    }
}

#[derive(Debug)]
struct Helicopter {
    speed: u8,
}

impl Accelerate for Helicopter {
    fn accelerate(&mut self) {
        self.speed += 50;
    }
}

impl Speed for Helicopter {
    fn speed(&self) -> u8 {
        self.speed
    }
}

impl Debug for dyn Accelerate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Accelerate")
    }
}

fn vector_heterogeneous() -> Vec<Box<dyn Vehicle>> {
    let mut vehicles: Vec<Box<dyn Vehicle>> = vec![];

    vehicles.push(Box::new(Airship { speed: 0 }));
    vehicles.push(Box::new(Bike { speed: 0 }));
    vehicles.push(Box::new(Helicopter { speed: 0 }));

    for vehicle in &mut vehicles {
        println!("Speed before: {:?}", vehicle.speed());
        vehicle.accelerate();
        println!("Speed after: {:?}", vehicle.speed());
    }

    vehicles
}

// VecDeque - double-ended queue implemented with a growable ring buffer
pub fn double_ended_queue() {
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
        println!("Overwriting {:?}", val);
    } else {
        println!("Writing initial value {:?}", s);
    }

    let s2 = "two";

    let val = h.entry(s2).or_insert(2);
    *val += 1;

    h.get(s2).copied()
}

pub fn hash_set<'a>() -> HashSet<&'a str> {
    let mut audio_tracks = HashSet::new();

    let tracks = ["track 1", "track 2", "track 3", "track 2", "track 1"];

    for track in tracks {
        audio_tracks.insert(track);
    }

    audio_tracks
}

// Iterator

struct AudioMarkers {
    current_marker: u32,
    next_marker: u32,
}

impl AudioMarkers {
    pub fn new() -> Self {
        Self {
            current_marker: 0,
            /// 10s
            next_marker: 10000,
        }
    }
}

impl Iterator for AudioMarkers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current_marker;

        self.current_marker = current + 10000;
        self.next_marker = self.current_marker + 10000;

        Some(current)
    }
}

type BoxedNode = Option<Box<Node>>;

#[derive(Debug, PartialEq)]
pub struct Node {
    value: u64,
    left: BoxedNode,
    right: BoxedNode,
}

#[derive(Debug)]
pub struct BST {
    root: BoxedNode,
    length: u64,
}

impl BST {
    pub fn add(&mut self, node: Node) {
        self.length += 1;
        let root = self.root.take();
        self.root = Self::add_recursively(root, node);
    }

    fn add_recursively(node: BoxedNode, new_node: Node) -> BoxedNode {
        match node {
            Some(mut node) => {
                if node.value >= new_node.value {
                    node.left = Self::add_recursively(node.left, new_node);
                    Some(node)
                } else {
                    node.right = Self::add_recursively(node.right, new_node);
                    Some(node)
                }
            }
            _ => Some(Box::new(new_node)),
        }
    }

    pub fn find(&mut self, value: u64) -> BoxedNode {
        let root = self.root.take();
        Self::find_recursively(root, value)
    }

    fn find_recursively(node: BoxedNode, value: u64) -> BoxedNode {
        match node {
            Some(node) if node.value > value => Self::find_recursively(node.left, value),
            Some(node) if node.value < value => Self::find_recursively(node.right, value),
            Some(node) => Some(node),
            None => None,
        }
    }
}

#[cfg(test)]
mod data_structures_tests {
    use super::*;

    #[test]
    fn vector_struct_sorting_works() {
        let actual = vector_struct_sort();

        assert!(actual[0].last_logged_in <= actual[1].last_logged_in);
    }

    #[test]
    fn vector_heterogeneous_works() {
        let actual = vector_heterogeneous();

        assert!(actual.len() == 3);
    }

    #[test]
    fn hash_map_works() {
        assert_eq!(hash_map(), Some(3));
    }

    #[test]
    fn hash_set_works() {
        let hs = hash_set();
        assert!(hs.len() == 3);
    }

    #[test]
    fn iterator_works() {
        let mut i = AudioMarkers::new();

        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), Some(10000));
        assert_eq!(i.next(), Some(20000));
    }

    #[test]
    fn bst_works() {
        let n = Node {
            value: 10,
            left: None,
            right: None,
        };
        let mut bst = BST {
            root: Some(Box::new(n)),
            length: 1,
        };

        let n2 = Node {
            value: 5,
            left: None,
            right: None,
        };
        bst.add(n2);

        let n3 = Node {
            value: 12,
            left: None,
            right: None,
        };
        bst.add(n3);

        let n4 = Node {
            value: 1,
            left: None,
            right: None,
        };
        bst.add(n4);

        assert!(bst.length == 4);
    }

    #[test]
    fn bst_find_works() {
        let n = Node {
            value: 10,
            left: None,
            right: None,
        };
        let mut bst = BST {
            root: Some(Box::new(n)),
            length: 1,
        };

        let n2 = Node {
            value: 5,
            left: None,
            right: None,
        };
        bst.add(n2);

        let n3 = Node {
            value: 12,
            left: None,
            right: None,
        };
        bst.add(n3);

        let n4 = Node {
            value: 1,
            left: None,
            right: None,
        };
        bst.add(n4);

        let node = bst.find(1);
        assert_eq!(
            node,
            Some(Box::new(Node {
                value: 1,
                left: None,
                right: None
            }))
        );
    }
}
