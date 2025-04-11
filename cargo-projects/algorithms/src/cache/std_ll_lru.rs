use std::{
    collections::{HashMap, LinkedList},
    hash::Hash,
};

struct LRUCache<'a, T> {
    data: LinkedList<T>,
    map: HashMap<T, Option<&'a T>>,
    capacity: usize,
}

impl<T: Eq + Hash + Clone> LRUCache<'_, T> {
    fn new(capacity: usize) -> Self {
        Self {
            data: LinkedList::new(),
            map: HashMap::new(),
            capacity,
        }
    }

    fn get(&self, val: &T) -> Option<&T> {
        let location = self.map.get(val);
        let loc = location?;
        *loc
    }

    fn put<'a>(&'a mut self, value: T) {
        // self.data.push_back(value.clone());
        // self.map.insert(value, self.data.back());
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
        let l = LRUCache::new(1);
        let actual = l.get(&"hello");

        assert_eq!(actual, None);
    }
}
