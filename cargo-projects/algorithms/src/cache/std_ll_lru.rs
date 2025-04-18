use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fmt::Debug,
    hash::Hash,
    rc::Rc,
};

#[derive(Debug)]
pub struct LRUCache<T> {
    data: Rc<RefCell<VecDeque<T>>>,
    map: RefCell<HashMap<T, usize>>,
    capacity: usize,
}

#[derive(Debug)]
struct Node<T> {
    prev: Option<usize>,
    next: Option<usize>,
    value: T,
}

#[derive(Debug)]
pub struct LRUCacheVec<T> {
    head: RefCell<Option<usize>>,
    tail: RefCell<Option<usize>>,
    capacity: usize,
    cache: RefCell<Vec<Option<Node<T>>>>,
    map: HashMap<T, usize>,
}

impl<T: Debug + Eq + PartialEq + Hash + Clone> LRUCacheVec<T> {
    fn new(capacity: usize) -> Self {
        Self {
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
    fn put(&mut self, value: T) {
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

    fn get(&self, value: &T)
    where
        T: PartialEq + Eq,
    {
        // self
        // let n
        // self.cache.push(

        // TODO
        // remove last element (tail)
        // let position = self.tail.unwrap();
        // let mut element = self.cache.get_mut(position).unwrap();

        // let prev = element.as_mut().unwrap().prev.unwrap();
        // let mut a = self.cache.get_mut(prev).unwrap();
        // a.as_mut().unwrap().next = None;

        // #2
        let mut value_index = None;
        for (index, val) in self.cache.borrow_mut().iter().enumerate() {
            if *value == val.as_ref().unwrap().value {
                value_index = Some(index);
                break;
            }
        }

        if value_index.is_none() {
            return;
        }

        let mut vec_cache = self.cache.borrow_mut();
        let element = vec_cache.get(value_index.unwrap()).unwrap();
        let prev_element_index = element.as_ref().unwrap().prev;
        let next_element_index = element.as_ref().unwrap().next;

        // let mut vec_cache_mut = vec_cache;
        if prev_element_index.is_some() {
            let prev_element_mut = vec_cache.get_mut(prev_element_index.unwrap()).unwrap();
            prev_element_mut.as_mut().unwrap().next = next_element_index;
        }

        if next_element_index.is_some() {
            let next_element_mut = vec_cache.get_mut(next_element_index.unwrap()).unwrap();
            next_element_mut.as_mut().unwrap().prev = prev_element_index;
        }

        // actual element
        let element = vec_cache.get_mut(value_index.unwrap()).unwrap();
        element.as_mut().unwrap().prev = None;
        element.as_mut().unwrap().next = Some(self.head.borrow().unwrap());
        let mut a = self.head.borrow_mut();
        *a = value_index;
    }

    fn lookup(&self, val: T) -> Option<T> {
        None
    }
}

impl<T: Eq + Hash + Clone + Debug> LRUCache<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Rc::new(RefCell::new(VecDeque::new())),
            map: RefCell::new(HashMap::new()),
            capacity,
        }
    }

    pub fn get(&self, val: &T) -> Option<T>
// where
    //     T: Borrow<T>,
    {
        // TODO, when we access the node we need to move it
        // to the head of the linked list since
        // it's last accessed element. For that we need a reference to Node.
        let mut map_mut = self.map.borrow_mut();
        let val_index = map_mut.get(val);
        println!("MAP {:?}", map_mut);
        let loc = val_index?;

        // Interior mutability
        let data_mut = self.data.clone();
        let mut data_mut = data_mut.borrow_mut();

        // let element = data_mut.get_mut(*loc);
        // let prev_element = element.unwrap();

        // let element = data_mut.remove(*loc);
        // data_mut.push_back(element.unwrap());

        let i = map_mut.insert(val.clone(), 0);
        // map_mut.insert(val.clone(), 0);

        println!("GET {:?}", data_mut);
        data_mut.front().cloned()

        // let data = self.data.clone();
        // let data_b = data.borrow();
        // let w = data_b.front();
        // w.cloned()

        // Ref::map(self.data.as_ref(), |n|)

        // Some(Ref::map(self.data.as_ref().borrow(), |n: &VecDeque<T>| {
        //     n.front().unwrap()
        // }))

        // let data = self.data.clone();

        // self.data.into_inner().front()

        // let a = self.data.as_ref();
        // a.borrow().front()
        // let w = a.get_mut();
        // w.front()

        // a

        // let f = Ref::map(self.data.clone().borrow(), |r| r.front())

        // let a = self.data.as_ref();
        // a.borrow().front()
    }

    pub fn put(&mut self, value: T) {
        // We are at capacity so remove LRU value
        let mut d = self.data.as_ref().borrow_mut();

        if self.capacity == d.len() {
            let key = d.pop_front();
            self.map.borrow_mut().remove(&key.unwrap());
        }

        // let mut d_2 = self.data.as_ptr().

        d.push_back(value.clone());
        // let item_position = 0;
        self.map.borrow_mut().insert(value, d.len() - 1);
    }
}

#[cfg(test)]
mod std_ll_lru_tests {
    use super::{LRUCache, LRUCacheVec};

    #[test]
    fn getting_non_existent_element_returns_none() {
        let l = LRUCache::new(1);
        let actual = l.get(&"hello");

        assert_eq!(actual.as_deref(), None);
    }

    // #[test]
    // fn putting_elements_returns_some() {
    //     let mut l = LRUCache::new(1);
    //     let actual = l.get(&"hello");
    //     assert_eq!(actual.as_deref(), None);

    //     // First element inserted
    //     l.put("hello");

    //     let b = l.get(&"hello");
    //     let item = b.as_deref();
    //     assert_eq!(item, Some(&"hello"));

    //     // Removes previous element since capacity is 1
    //     l.put("hi");
    //     let item = l.get(&"hi").as_deref();
    //     assert!(l.data.borrow().len() == 1);
    //     assert_eq!(item, Some(&"hi"));
    // }

    #[test]
    fn putting_more_elements_than_capacity_removes_first_element() {
        let mut cache = LRUCache::new(5);
        cache.put("one");
        cache.put("two");
        cache.put("three");
        cache.put("four");
        cache.put("five");
        cache.put("six");

        // Capacity is 5, so first element gets removed
        let first_element = cache.get(&"one");
        assert_eq!(first_element, None);
    }

    #[test]
    fn putting_more_elements_than_capacity_removes_least_used_element() {
        let mut cache = LRUCache::new(5);
        cache.put("one");
        cache.put("two");
        cache.put("three");
        cache.put("four");
        cache.put("five");

        cache.get(&"one");

        // cache.put("six");

        // Capacity is 5, so first element gets removed
        let first_element: Option<&str> = cache.get(&"one");
        assert_eq!(first_element, Some("one"));
    }

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

    fn lru_get_works_without_values() {
        // let mut cache = LRUCacheVec::new(5);

        // assert_eq!(cache.get(&1), None);
    }
}
