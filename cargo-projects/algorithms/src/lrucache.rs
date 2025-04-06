use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
struct LinkedList<T: Clone> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    val: Option<T>,
}

#[derive(Debug, Clone)]
struct Node<T>
where
    T: std::clone::Clone,
{
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
    val: Option<T>,
}

impl<T: Clone> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            val: None,
        }
    }

    fn append(&mut self, val: T) -> Option<Box<Node<T>>> {
        let node = Node {
            prev: None,
            next: None,
            val: Some(val.clone()),
        };

        if self.head.is_none() {
            self.head = Some(Box::new(node.clone()));
            self.tail = Some(Box::new(node.clone()));
            return self.head.clone();
        }

        // take tail
        let mut tail = self.tail.clone().unwrap();

        let Some(tail_previous) = tail.prev.as_mut() else {
            tail.prev = self.head.clone();
            tail.val = Some(val.clone());
            let new_node = Some(Box::new(*tail));
            self.tail = new_node.clone();
            return new_node;
        };

        tail_previous.next = Some(Box::new(node.clone()));
        let w = &tail_previous.as_mut().next;
        let mut p = w.clone().unwrap().next;
        p = Some(Box::new(*tail));
        p
    }
}

#[derive(Debug)]
struct LRU<T: Clone> {
    map: HashMap<String, Option<Box<Node<T>>>>,
    linked_list: LinkedList<T>,
}

// Doubly LinkedList - to track most frequently accessed items (head) and least frequently accessed items (tail)

// Hashmap - to get location in the linked list instantly

impl<T: Clone + Debug> LRU<T> {
    fn new(capacity: u32) -> Self {
        LRU {
            map: HashMap::new(),
            linked_list: LinkedList::new(),
        }
    }

    fn get(&self, key: &str) -> Option<T> {
        let hash = self.map.get(key);

        if let Some(h) = hash {
            if let Some(v) = h {
                return v.clone().val;
            }
        }

        return None;
    }

    fn put(&mut self, key: &str, value: T) {
        let n = self.linked_list.append(value);
        self.map.insert(key.to_string(), n);
    }
}

#[cfg(test)]
mod lru_tests {
    use super::LRU;

    #[test]
    fn works() {
        let lru: LRU<usize> = LRU::new(10);
        let actual = lru.get("hello");

        assert!(actual.is_none());
    }

    #[test]
    fn works_2() {
        let mut lru: LRU<usize> = LRU::new(10);
        let actual = lru.get("hello");

        assert!(actual.is_none());

        lru.put("key", 100);

        assert_eq!(lru.get("key"), Some(100));

        lru.put("key-2", 13);

        assert_eq!(lru.get("key-2"), Some(13));
    }
}
