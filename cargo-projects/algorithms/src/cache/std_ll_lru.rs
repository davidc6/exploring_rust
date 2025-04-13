use std::{
    borrow::Borrow,
    collections::{HashMap, LinkedList},
    hash::Hash,
};

struct LRUCache<T> {
    data: LinkedList<T>,
    map: HashMap<T, Option<T>>,
    capacity: usize,
}

impl<T: Eq + Hash + Clone> LRUCache<T> {
    fn new(capacity: usize) -> Self {
        Self {
            data: LinkedList::new(),
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
        location?.as_ref()
    }

    fn put(&mut self, value: T) {
        // We are at capacity so remove LRU value
        if self.capacity == self.data.len() {
            self.data.pop_back();
        }

        self.data.push_back(value.clone());
        let node = self.data.back();
        self.map.insert(value, node.cloned());
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

        l.put("hello");
        let g = l.get(&"hello").cloned();

        assert_eq!(g, Some(&"hello").map(|v| &**v));
    }
}
