use std::{borrow::Borrow, cell::RefCell, collections::HashMap, fmt::Debug, hash::Hash};

#[derive(Debug)]
pub struct Node<T> {
    prev: Option<usize>,
    next: Option<usize>,
    value: T,
}

pub struct LRUCacheVec<T> {
    head: RefCell<Option<usize>>,
    tail: RefCell<Option<usize>>,
    capacity: usize,
    cache: RefCell<Vec<Option<Node<T>>>>,
    map: HashMap<T, usize>,
}

impl<T: Debug + Eq + PartialEq + Hash + Clone> LRUCacheVec<T> {
    pub fn new(capacity: usize) -> Self {
        LRUCacheVec {
            head: RefCell::new(None),
            tail: RefCell::new(None),
            cache: RefCell::new(Vec::new()),
            map: HashMap::new(),
            capacity,
        }
    }

    /// Inserts a new value at the head of the list/cache.
    ///
    /// The newly inserted value will be the most recently accessed one.
    pub fn put(&mut self, value: T) {
        // New node/entry
        let mut node = Node {
            prev: None,
            next: None,
            value: value.clone(),
        };

        // #1 - The list is empty
        if self.head.borrow().is_none() {
            let mut cache_mut = self.cache.borrow_mut();
            cache_mut.push(Some(node));

            let index = cache_mut.len() - 1;

            // Update HashMap
            self.map.insert(value, index);

            // head and tail as the same index since no items in the list yet
            self.head = RefCell::new(Some(index));
            self.tail = RefCell::new(Some(index));

            return;
        }

        // #2 - The list is filled
        if self.cache.borrow().len() == self.capacity {
            let mut cache_mut = self.cache.borrow_mut();

            // Since there are values in the list,
            // We know that there is a tail so we can unwrap here.
            let head_index = self.head.borrow().unwrap();
            let tail_index = self.tail.borrow().unwrap();

            // New node is head so nothing to link to previously
            node.prev = None;
            node.next = Some(head_index);

            // TODO, deal with the previous node - remove tail
            // 1. get previous node, which becomes tail now and set next to None
            // let previous_tail = cache_mut.get_mut(tail_index).unwrap();

            // get previous to tail element index
            let index_of_prev_to_tail_element = if let Some(Some(v)) = cache_mut.get_mut(tail_index)
            {
                v.prev.unwrap_or(0)
            } else {
                0
            };

            // set previous to tail element next to None
            if let Some(val_node) = cache_mut.get_mut(index_of_prev_to_tail_element) {
                if let Some(val) = val_node {
                    val.next = None;
                }
            } else {
                return;
            };

            // set tail to new element
            self.tail = RefCell::new(Some(index_of_prev_to_tail_element));

            if let Some(Some(val)) = cache_mut.get_mut(head_index) {
                val.prev = Some(tail_index);
            }

            if let Some(val) = cache_mut.get_mut(tail_index) {
                let key = &val.as_ref().unwrap().value;
                self.map.remove(key);
                self.map.insert(value, tail_index);

                *val = Some(node);
            }

            self.head = RefCell::new(Some(tail_index));

            return;
        }

        // #3 - The list has space
        let mut cache_mut = self.cache.borrow_mut();
        let index = cache_mut.len();

        let current_node = cache_mut
            .get_mut(self.head.borrow().unwrap())
            .unwrap()
            .as_mut();

        // new head
        node.prev = None;
        node.next = Some(self.head.borrow().unwrap());

        // previous head should have previous set to new head index
        if let Some(node_mut) = current_node {
            node_mut.prev = Some(index);
        };

        // Update HashMap
        self.map.insert(value, index);

        cache_mut.push(Some(node));

        self.head = RefCell::new(Some(index));

        // cache_mut[position].as_mut().unwrap().next = *self.head.borrow();
        // self.head = RefCell::new(Some(position));
    }

    pub fn get(&self, value: &T) -> Option<T>
    where
        T: PartialEq + Eq + Borrow<T>,
    {
        let Some(index) = self.map.get(value) else {
            return None;
        };

        let mut value_to_return = None;

        // Get current element's previous and next node indices
        let (previous_index, next_index) = {
            let mut cache = self.cache.borrow();

            if let Some(elem) = cache.get(*index) {
                let previous_element_index = elem.as_ref().unwrap().prev;
                let next_element_index = elem.as_ref().unwrap().next;
                (previous_element_index, next_element_index)
            } else {
                (None, None)
            }
        };

        // Update previous element's next index (to contain current element's next element index)
        if let Some(prev_index) = previous_index {
            let mut cache = self.cache.borrow_mut();
            if let Some(Some(elem)) = cache.get_mut(prev_index) {
                elem.next = next_index;
            }
        }

        // Update next element's prev index (to contain current element's prev element index)
        if let Some(nxt_index) = next_index {
            let mut cache = self.cache.borrow_mut();
            if let Some(Some(elem)) = cache.get_mut(nxt_index) {
                elem.prev = previous_index;
            }
        }

        // Set current element to become head
        let mut cache = self.cache.borrow_mut();
        if let Some(Some(elem)) = cache.get_mut(*index) {
            {
                elem.prev = None;
                elem.next = *self.head.borrow();
                value_to_return = Some(elem.value.clone());
            }
        }

        // Update head
        let mut head_mut = self.head.borrow_mut();
        *head_mut = Some(*index);

        value_to_return
    }

    pub fn head(&self) -> Option<usize> {
        *self.head.borrow()
    }

    pub fn tail(&self) -> Option<usize> {
        *self.tail.borrow()
    }

    fn lookup(&self, val: T) -> Option<T> {
        None
    }
}

#[cfg(test)]
mod std_ll_lru_tests {
    use super::LRUCacheVec;

    #[test]
    fn lru_2() {
        let mut cache = LRUCacheVec::new(5);
        cache.put(1);
        cache.put(2);
        cache.put(3);
        cache.put(4);
        cache.put(5);
        cache.put(6); // should remove 1
                      // cache.put(7); // should remove 2

        for val in cache.cache.borrow().iter() {
            println!("{:?}", val);
        }

        println!("HEAD {:?}", cache.head);
        println!("TAIL {:?}", cache.tail);

        println!("MAP {:?}", cache.map);

        // cache.get(&3);
    }

    #[test]
    fn lru_works_when_empty_cache() {
        let cache = LRUCacheVec::new(5);

        assert_eq!(cache.get(&1), None);
    }

    #[test]
    fn lru_works_when_single_item_is_put_in_cache() {
        let mut cache = LRUCacheVec::new(5);
        cache.put(1);

        assert_eq!(cache.get(&1), Some(1));
    }

    #[test]
    fn lru_works_when_at_capacity() {
        let mut cache = LRUCacheVec::new(5);
        let v: Vec<u32> = (1..=5).collect();

        for val in v.iter() {
            cache.put(val);
        }

        for val in v.iter() {
            assert_eq!(cache.get(&val), Some(val));
        }
    }

    #[test]
    fn lru_works_when_over_capacity() {
        let mut cache = LRUCacheVec::new(5);
        let v: Vec<u32> = (1..=5).collect();

        for val in v.iter() {
            cache.put(val);
        }

        cache.put(&6);

        let expected = [6, 2, 3, 4, 5];

        for val in expected.iter() {
            assert_eq!(cache.get(&val), Some(val));
        }

        cache.put(&7);

        let expected = [7, 6, 3, 4, 5];

        for val in expected.iter() {
            assert_eq!(cache.get(&val), Some(val));
        }
    }

    #[test]
    fn lru_works_when_completely_replaced_by() {
        let mut cache = LRUCacheVec::new(5);

        // Initial 1 to 5 value insertion
        let v: Vec<u32> = (1..=5).collect();

        for val in v.iter() {
            cache.put(val);
        }

        for val in v.iter() {
            assert_eq!(cache.get(&val), Some(val));
        }

        // Follow up with 6 to 10 values to completely replace the cache with new values
        let v: Vec<u32> = (6..=10).collect();

        for val in v.iter() {
            cache.put(val);
        }

        let expected = [10, 9, 8, 7, 6];

        for val in expected.iter() {
            assert_eq!(cache.get(&val), Some(val));
        }
    }

    #[test]
    fn lru_works_on_get_operation() {
        let mut cache = LRUCacheVec::new(5);
        let v: Vec<u32> = (1..=5).collect();

        for val in v.into_iter() {
            cache.put(val);
        }

        let head = cache.head();
        let tail = cache.tail();

        assert_eq!(head, Some(4));
        assert_eq!(tail, Some(0));

        // Calling get() method updates the head index
        cache.get(&3);

        let head = cache.head();
        let tail = cache.tail();

        assert_eq!(head, Some(2));
        assert_eq!(tail, Some(0));
    }
}
