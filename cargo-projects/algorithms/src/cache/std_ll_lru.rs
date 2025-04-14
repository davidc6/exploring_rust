use std::{
    borrow::Borrow,
    collections::{HashMap, VecDeque},
    hash::Hash,
};

struct LRUCache<T> {
    // data: LinkedList<Rc<T>>,
    data: VecDeque<T>,
    map: HashMap<T, usize>,
    capacity: usize,
}

impl<T: Eq + Hash + Clone> LRUCache<T> {
    fn new(capacity: usize) -> Self {
        Self {
            data: VecDeque::new(),
            map: HashMap::new(),
            capacity,
        }
    }

    fn get(&self, val: &T) -> Option<&T>
    where
        T: Borrow<T>,
    {
        // TODO, when we access the node we need to move it
        // to the head of the linked list since
        // it's last accessed element. For that we need a reference to Node.
        let location = self.map.get(val);
        let loc = location?;
        self.data.get(*loc)
    }

    fn put(&mut self, value: T) {
        // We are at capacity so remove LRU value
        if self.capacity == self.data.len() {
            self.data.pop_back();
        }

        self.data.push_back(value.clone());
        let item_position = self.data.len() - 1;
        self.map.insert(value, item_position);
    }
}

#[cfg(test)]
mod std_ll_lru_tests {
    use super::LRUCache;

    #[test]
    fn works() {
        let l = LRUCache::new(1);
        let actual = l.get(&"hello");

        assert_eq!(actual, None);
    }

    #[test]
    fn works_2() {
        let mut l = LRUCache::new(1);
        let actual = l.get(&"hello");
        assert_eq!(actual, None);

        // First element inserted
        l.put("hello");
        let item = l.get(&"hello").cloned();
        assert_eq!(item, Some("hello"));

        // Removes previous element since capacity is 1
        l.put("hi");
        let item = l.get(&"hi").cloned();
        assert!(l.data.len() == 1);
        assert_eq!(item, Some("hi"));
    }
}
