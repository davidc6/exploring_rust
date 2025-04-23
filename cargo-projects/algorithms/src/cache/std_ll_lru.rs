use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    collections::HashMap,
    default,
    fmt::Debug,
    hash::Hash,
    rc::Rc,
};

type Entry<T> = Option<Rc<RefCell<Node<T>>>>;
type Position = Option<Rc<RefCell<usize>>>;

#[derive(Debug)]
pub struct Node<T> {
    prev: Position,
    next: Position,
    value: T,
}

/// LRUCache struct holds cache data and
/// information related to it, such as
/// head and tail pointers (least and most
/// recently accessed items), cache capacity.
#[derive(Debug)]
pub struct LRUCache<T> {
    head: RefCell<Option<usize>>,
    tail: RefCell<Option<usize>>,
    capacity: usize,
    cache: RefCell<Vec<Option<Node<T>>>>,
    map: HashMap<T, usize>,
}

impl<T: Clone + Debug + Eq + Hash + PartialEq> Default for LRUCache<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Debug + Eq + Hash + PartialEq> LRUCache<T> {
    pub fn new() -> Self {
        Self::with_capacity(10)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        LRUCache {
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

            if self.capacity == 1 {
                // TODO
            }

            // Since there are values in the list,
            // We know that there is a tail so we can unwrap here.
            let head_index = self.head.borrow().unwrap();
            let tail_index = self.tail.borrow().unwrap();

            // println!("HEAD {:?} TAIL {:?}", head_index, tail_index);

            // New node is head so nothing to link to previously
            node.prev = None;
            node.next = Some(Rc::new(RefCell::new(head_index)));

            // TODO, deal with the previous node - remove tail
            // 1. get previous node, which becomes tail now and set next to None
            // let previous_tail = cache_mut.get_mut(tail_index).unwrap();

            // get previous to tail element index
            let tail_prev_index = if let Some(Some(elem)) = cache_mut.get_mut(tail_index) {
                // If there's only one element in cache then it's head and tail,
                // therefore prev and next are None.
                elem.prev.clone().unwrap_or(Rc::new(RefCell::new(0)))
            } else {
                Rc::new(RefCell::new(0))
            };

            // println!("")
            // println!("TAIL PREV {:?}", tail_prev_index);

            // let index_of_prev_to_tail_element = tail_prev.as_ref().unwrap().prev.unwrap();

            // set previous to tail element next to None (since we are removing the tail element)
            // println!("TAIL TAIL {:?}", tail_prev_index);

            let tail_prev_index = tail_prev_index.clone().take();
            if let Some(Some(val_node)) = cache_mut.get_mut(tail_prev_index) {
                val_node.next = None;
            } else {
                return;
            };

            // let next_to_head_element = cache_mut.get_mut(head_index).unwrap();
            // let index_of_next_to_head_element =

            // previous to tail (next to none)
            // previous head prev to head

            // set tail to new element
            self.tail = RefCell::new(Some(tail_prev_index));

            if let Some(Some(val)) = cache_mut.get_mut(head_index) {
                val.prev = Some(Rc::new(RefCell::new(tail_index)));
            }

            // Update with the new node
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
        let index = cache_mut.len(); // new head location

        // let current_node = cache_mut
        //     .get_mut(self.head.borrow().unwrap())
        //     .unwrap()
        //     .as_mut();

        // new head
        let head = self.head.borrow().unwrap();
        node.prev = None;
        node.next = Some(Rc::new(RefCell::new(head)));

        // previous head should have previous set to new head index
        if let Some(Some(node_mut)) = cache_mut.get_mut(self.head.borrow().unwrap()) {
            node_mut.prev = Some(Rc::new(RefCell::new(index)));
            // node_mut.next = node.
        };

        // Update HashMap
        self.map.insert(value, index);
        cache_mut.push(Some(node));
        self.head = RefCell::new(Some(index));
    }

    pub fn get(&self, value: &T) -> Option<Ref<T>>
    where
        T: PartialEq + Eq + Borrow<T>,
    {
        let index = self.map.get(value)?;

        // Get current element's previous and next node indices
        let (previous_index, next_index) = {
            let cache = self.cache.borrow();

            if let Some(elem) = cache.get(*index) {
                let elem_ref = elem.as_ref().unwrap();
                let previous_element_index = elem_ref.prev.clone();
                let next_element_index = elem_ref.next.clone();

                (previous_element_index, next_element_index)
            } else {
                (None, None)
            }
        };

        // Update previous element's next index (to contain current element's next element index)
        if let Some(ref prev_index) = previous_index {
            let mut cache = self.cache.borrow_mut();
            if let Some(Some(elem)) = cache.get_mut(*prev_index.borrow_mut()) {
                elem.prev = Some(Rc::new(RefCell::new(*index)));
                elem.next = None;
            }
        }

        // Update next element's prev index (to contain current element's prev element index)
        if let Some(nxt_index) = next_index {
            let mut cache = self.cache.borrow_mut();
            let n = nxt_index.clone().take();
            if let Some(Some(elem)) = cache.get_mut(n) {
                elem.prev = Some(Rc::new(RefCell::new(*index)));
            }
        }

        // HEAD - Set current element to become head
        {
            let mut cache = self.cache.borrow_mut();
            if let Some(Some(elem)) = cache.get_mut(*index) {
                let h = self.head.borrow().unwrap();
                elem.prev = None;
                elem.next = Some(Rc::new(RefCell::new(h)));
                // value_to_return = Some(elem.value.clone());
            }
        }

        // Update head
        let mut head_mut = self.head.borrow_mut();
        *head_mut = Some(*index);

        // Update tail
        let mut tail_mut = self.tail.borrow_mut();
        let i = previous_index.clone().unwrap_or(Rc::new(RefCell::new(0)));
        *tail_mut = Some(i.clone().take());

        let elem = self.cache.borrow();
        if let Some(Some(_)) = elem.get(*index) {
            Some(Ref::map(elem, |e| {
                &e.get(*index).unwrap().as_ref().unwrap().value
            }))
        } else {
            None
        }
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

struct LRUCacheState<'a, T>
where
    T: 'a,
{
    current_element: Option<usize>,
    lru_cache: Rc<Ref<'a, Vec<Option<Node<T>>>>>,
}

impl<'a, T: Hash + Eq + PartialEq + Debug + Clone + 'a> LRUCacheState<'a, T> {
    fn new(lru_cache: &'a LRUCache<T>) -> Self {
        let data = Rc::new(lru_cache.cache.borrow());

        LRUCacheState {
            current_element: lru_cache.head(),
            lru_cache: data,
        }
    }
}

impl<'a, T: 'a> Iterator for LRUCacheState<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let old_index = self.current_element.unwrap();
        let w = self.lru_cache.get(old_index);

        let res = if let Some(Some(el)) = w {
            Some(el)
        } else {
            None
        };

        self.current_element = Some(self.current_element.unwrap() + 1);

        // res <---- FIX THIS
        // res
        None
    }
}

#[cfg(test)]
mod std_ll_lru_tests {
    use super::LRUCache;

    #[test]
    fn lru_works_when_empty_cache() {
        let cache = LRUCache::with_capacity(5);
        let a = cache.get(&1);

        assert!(a.is_none());
    }

    #[test]
    fn lru_works_when_single_item_is_put_in_cache() {
        let mut cache = LRUCache::with_capacity(5);
        cache.put(1);

        let actual = *cache.get(&1).unwrap();

        assert!(actual == 1);
    }

    #[test]
    fn lru_works_when_at_capacity() {
        let mut cache = LRUCache::with_capacity(5);
        let v: Vec<u32> = (1..=5).collect();

        for val in v.iter() {
            cache.put(val);
        }

        for val in v.iter() {
            let a = *cache.get(&val).unwrap();
            assert_eq!(a, val);
        }
    }

    #[test]
    fn lru_works_when_over_capacity() {
        let mut cache = LRUCache::with_capacity(5);
        let v: Vec<u32> = (1..=5).collect();

        for val in v.iter() {
            cache.put(val);
        }

        cache.put(&6);

        let expected = [6, 2, 3, 4, 5];

        for val in expected.iter() {
            assert!(*cache.get(&val).unwrap() == val);
        }

        cache.put(&7);

        let expected = [7, 6, 3, 4, 5];

        for val in expected.iter() {
            assert!(*cache.get(&val).unwrap() == val);
        }
    }

    #[test]
    fn lru_works_when_completely_replaced_by() {
        let mut cache = LRUCache::with_capacity(3);

        // Initial 1 to 5 value insertion
        let v: Vec<u32> = (1..=3).collect();

        for val in v.iter() {
            cache.put(val);
        }

        println!("HEAD {:?} TAIL {:?}", cache.head, cache.tail);
        println!("OLD MAP {:?}", cache.cache);

        for val in v.iter() {
            assert!(*cache.get(&val).unwrap() == val);
        }

        // for val in cache.cache.into_inner() {
        //     println!("VALUE {:?}", val);
        // }

        // Follow up with 6 to 10 values to completely replace the cache with new values
        let v: Vec<u32> = (4..=6).collect();

        for val in v.iter() {
            cache.put(val);
        }

        println!("=========================================================================");

        println!("HEAD {:?} TAIL {:?}", cache.head, cache.tail);
        println!("NEW MAP {:?}", cache.cache);

        // println!("MAP {:?}", cache.cache);

        // for val in cache.cache.into_inner() {
        //     println!("VALUE {:?}", val);
        // }

        // let expected = [10, 9, 8, 7, 6];

        // for val in expected.iter() {
        //     println!("VALUE {:?}", val);
        //     assert!(*cache.get(&val).unwrap() == val);
        // }
    }

    #[test]
    fn lru_works_on_get_operation() {
        let mut cache = LRUCache::with_capacity(5);
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

    #[test]
    fn lru_works_on_get_operation_next() {
        let mut cache = LRUCache::with_capacity(5);
        let v: Vec<u32> = (1..=5).collect();

        for val in v.into_iter() {
            cache.put(val);
        }

        // for value in cache.cache.into_inner() {}

        // let head = cache.head();
        // let tail = cache.tail();

        // assert_eq!(head, Some(4));
        // assert_eq!(tail, Some(0));

        // // Calling get() method updates the head index
        // cache.get(&3);

        // let head = cache.head();
        // let tail = cache.tail();

        // assert_eq!(head, Some(2));
        // assert_eq!(tail, Some(0));
    }
}
