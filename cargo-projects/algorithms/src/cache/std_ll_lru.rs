use std::{
    borrow::Borrow,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    ops::{Deref, DerefMut},
    rc::Rc,
    // slice::IterMut,
};

type Entry<T> = Option<Rc<RefCell<Node<T>>>>;
type Position = Option<Rc<RefCell<usize>>>;

#[derive(Debug)]
pub struct Node<T> {
    prev: Position,
    next: Position,
    value: T,
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
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
    cache: Vec<Rc<RefCell<Option<Node<T>>>>>,
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
            cache: Vec::new(),
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
            // let l = self.cache;
            // let mut cache_mut = self.cache;
            self.cache.push(Rc::new(RefCell::new(Some(node))));

            let index = self.cache.len() - 1;

            // Update HashMap
            self.map.insert(value, index);

            // head and tail as the same index since no items in the list yet
            self.head = RefCell::new(Some(index));
            self.tail = RefCell::new(Some(index));

            return;
        }

        // #2 - The list is filled
        if self.cache.len() == self.capacity {
            if self.capacity == 1 {
                // TODO
            }

            // Since there are values in the list,
            // We know that there is a tail so we can unwrap here.
            let head_index = self.head.borrow().unwrap();
            let tail_index = self.tail.borrow().unwrap();

            // New node is head so nothing to link to previously
            node.prev = None;
            node.next = Some(Rc::new(RefCell::new(head_index)));

            // Get previous to tail element index
            // let tail_prev_index = if let Some(Some(elem)) = cache_mut.get_mut(tail_index) {
            let tail_prev = self.cache.get_mut(tail_index);
            let tail_prev_index = if let Some(elem) = tail_prev {
                // If there's only one element in cache then it's head and tail,
                // therefore prev and next are None.
                let b = elem.clone();
                let b = &*b;
                let p = b.borrow();
                let p = p.as_ref().unwrap().prev.clone();
                p.unwrap_or(Rc::new(RefCell::new(0)))
            } else {
                Rc::new(RefCell::new(0))
            };

            let tail_prev_index = tail_prev_index.clone().take();

            if let Some(val_node) = self.cache.get_mut(tail_prev_index) {
                let mut v = val_node.clone();
                let mut v = v.borrow_mut();
                let v = v.as_mut().unwrap();
                v.next = None;
            } else {
                return;
            };

            // Set tail to new element
            self.tail = RefCell::new(Some(tail_prev_index));

            if let Some(val) = self.cache.get_mut(head_index) {
                let v = val.clone();
                let mut v = v.borrow_mut();
                let v = v.as_mut().unwrap();
                v.prev = Some(Rc::new(RefCell::new(tail_index)));
            }

            // Update with the new node
            if let Some(val) = self.cache.get_mut(tail_index) {
                let v = val.clone();
                let v = v.as_ref().borrow();
                let v = v.as_ref();
                let key = &v.unwrap().value;

                self.map.remove(key);
                self.map.insert(value, tail_index);

                *val = Rc::new(RefCell::new(Some(node)));
            }

            self.head = RefCell::new(Some(tail_index));

            return;
        }

        // #3 - The list has space
        let index = self.cache.len(); // new head location

        // new head
        let head = self.head.borrow().unwrap();
        node.prev = None;
        node.next = Some(Rc::new(RefCell::new(head)));

        // previous head should have previous set to new head index
        if let Some(node_mut) = self.cache.get_mut(self.head.borrow().unwrap()) {
            let v = node_mut.clone();
            let mut v = v.borrow_mut();
            let v = v.as_mut().unwrap();
            v.prev = Some(Rc::new(RefCell::new(index)));
        };

        // Update HashMap
        self.map.insert(value, index);
        self.cache.push(Rc::new(RefCell::new(Some(node))));
        self.head = RefCell::new(Some(index));
    }

    pub fn get(&self, value: &T) -> Option<T>
    where
        T: PartialEq + Eq + Borrow<T>,
    {
        let index = self.map.get(value)?;

        // Get current element's previous and next node indices
        let (previous_index, next_index) = {
            let cache = self.cache.clone();

            if let Some(elem) = cache.get(*index) {
                let elem = elem.clone();
                let mut elem_ref = elem.borrow_mut();
                let elem_ref = elem_ref.as_mut().unwrap();

                // let elem_ref = elem.as_ref().unwrap();
                let previous_element_index = elem_ref.prev.clone();
                let next_element_index = elem_ref.next.clone();

                (previous_element_index, next_element_index)
            } else {
                (None, None)
            }
        };

        // Update previous element's next index (to contain current element's next element index)
        if let Some(ref prev_index) = previous_index {
            let mut cache = self.cache.clone();
            let w = prev_index.clone();
            let i = *w.as_ref().borrow();
            // let i = *i.borrow_mut();
            if let Some(elem) = cache.get_mut(i) {
                let elem = elem.clone();
                let mut elem_ref = elem.borrow_mut();
                let elem = elem_ref.as_mut().unwrap();

                elem.prev = Some(Rc::new(RefCell::new(*index)));
                elem.next = None;
            }
        }

        // Update next element's prev index (to contain current element's prev element index)
        if let Some(nxt_index) = next_index {
            let mut cache = self.cache.clone();
            let n = nxt_index.clone().take();
            if let Some(elem) = cache.get_mut(n) {
                let elem = elem.clone();
                let mut elem_ref = elem.borrow_mut();
                let elem = elem_ref.as_mut().unwrap();

                elem.prev = Some(Rc::new(RefCell::new(*index)));
            }
        }

        // HEAD - Set current element to become head
        {
            let mut cache = self.cache.clone();
            if let Some(elem) = cache.get_mut(*index) {
                let h = self.head.borrow().unwrap();
                let elem = elem.clone();
                let mut elem = elem.borrow_mut();
                let elem = elem.as_mut().unwrap();

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

        let cache = self.cache.clone();

        let a = cache.get(*index);
        let a = a.unwrap();
        let a = a.as_ref().borrow();
        let a = a.as_ref().unwrap().value.clone();
        Some(a)

        // if let Some(elem) = cache.get(*index) {
        //     // let elem = elem.clone();

        //     // let elem = elem;
        //     // let elem = *elem;
        //     let elem = elem.as_ref().borrow();
        //     // let elem_ref = elem_ref.as_mut().unwrap();

        //     Some(Ref::map(elem, |e| &e.unwrap().value))
        // } else {
        //     None
        // }
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

    // pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    //     let head_index = self.head.borrow().unwrap();
    //     // let head_mut = self.cache.get_mut();
    //     let vec_mut = self.cache.get_mut();
    //     let mut node_mut = vec_mut.get_mut(head_index).unwrap().as_mut();
    //     // let mut node_mut = vec_mut.get_mut(head_index).unwrap().iter_mut();

    //     // vec_mut

    //     IterMut {
    //         this: &mut node_mut,
    //     }
    // }
}

pub struct IterMut<T> {
    // A wrapper for mutably borrowed value (from RefCell)
    this: Vec<Rc<RefCell<Option<Node<T>>>>>,
    count: usize,
    head: usize,
}

impl<'a, T> IntoIterator for &'a mut LRUCache<T> {
    type Item = Rc<RefCell<Option<Node<T>>>>;
    type IntoIter = IterMut<T>;

    fn into_iter(self) -> Self::IntoIter {
        // We mutably borrow from RefCell here
        IterMut {
            this: self.cache.clone(),
            count: 0,
            head: self.head.borrow().unwrap(),
        }
    }
}

impl<T> Iterator for IterMut<T> {
    // We want to get a mutable reference to Node
    type Item = Rc<RefCell<Option<Node<T>>>>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO - this is impossible to achieve since
        // let mut node = self.this;

        // Some(Ref::map(self.this, |e| {
        //     let a = e.get(0);
        //     a.unwrap()
        // }))

        // let node = node.get_mut(1).unwrap().as_mut();
        // node
        // None

        let old_count = self.count;
        self.count += 1;
        let head = self.head;

        let n = self.this.clone();

        if n.len() == old_count {
            return None;
        }

        let w = n.get(head).unwrap().clone();
        let w_1 = w.clone();
        let w_2 = w_1.as_ref();
        let w_21 = &w_2.borrow();
        let w_22 = w_21.as_ref().unwrap();
        let w_3 = w_22.next.as_ref();

        if w_3.is_none() {
            Some(w)
        } else {
            let w_3 = w_3.unwrap();
            let w_4 = w_3.as_ref();
            let w_5 = w_4.borrow();
            let w_6 = *w_5;

            self.head = w_6;

            Some(w)
        }
    }
}

// struct LRUCacheState<'a, T>
// where
//     T: 'a,
// {
//     current_element: Option<usize>,
//     lru_cache: Rc<Ref<'a, Vec<Option<Node<T>>>>>,
// }

// impl<'a, T: Hash + Eq + PartialEq + Debug + Clone + 'a> LRUCacheState<'a, T> {
//     fn new(lru_cache: &'a LRUCache<T>) -> Self {
//         let data = Rc::new(lru_cache.cache.borrow());

//         LRUCacheState {
//             current_element: lru_cache.head(),
//             lru_cache: data,
//         }
//     }
// }

// impl<'a, T: 'a> Iterator for LRUCacheState<'a, T> {
//     type Item = &'a Node<T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let old_index = self.current_element?;
//         let element = self.lru_cache.get(old_index)?;

//         let res = if let Some(el) = element {
//             Some(el)
//         } else {
//             None
//         };

//         self.current_element = Some(self.current_element.unwrap() + 1);

//         // res <---- FIX THIS
//         res
//         // None
//     }
// }

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

        let actual = cache.get(&1).unwrap();

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
            let a = cache.get(&val).unwrap();
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
            assert!(cache.get(&val).unwrap() == val);
        }

        cache.put(&7);

        let expected = [7, 6, 3, 4, 5];

        for val in expected.iter() {
            assert!(cache.get(&val).unwrap() == val);
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
            assert!(cache.get(&val).unwrap() == val);
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

        // println!("BOLOG {:?}", cache);

        for val in &mut cache {
            // println!("IN LOOP ");
            // let a = val.as_mut().unwrap();
            // a.value;
            println!("Node {:?}", val);
            // println!("BEFORE {:?}", val.as_ref().unwrap().value);
            // val.as_mut().unwrap().value = 11;
            // println!("AFTER {:?}", val.as_ref().unwrap().value);
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
