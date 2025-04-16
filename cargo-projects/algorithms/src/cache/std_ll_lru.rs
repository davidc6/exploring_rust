use std::{
    // borrow::Borrow,
    cell::{Ref, RefCell},
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
}

impl<T: Debug> LRUCacheVec<T> {
    fn new(capacity: usize) -> Self {
        Self {
            head: RefCell::new(None),
            tail: RefCell::new(None),
            capacity,
            cache: RefCell::new(Vec::new()),
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
            value,
        };

        // #1 - The list is empty.
        if self.head.borrow().is_none() {
            let mut m = self.cache.borrow_mut();
            m.push(Some(node));

            let position = m.len() - 1;

            // head and tail as the same index
            self.head = RefCell::new(Some(position));
            self.tail = RefCell::new(Some(position));
            return;
        }

        // list is filled
        if self.cache.borrow().len() == self.capacity {
            //
            let tail_index = self.tail.borrow().unwrap();
            let mut cache_mut = self.cache.borrow_mut();
            let tail = cache_mut.get_mut(tail_index).unwrap();

            let new_tail_index = tail.as_ref().unwrap().prev.unwrap();
            // self.cache.swap_remove(tail_index);
            node.prev = None;
            node.next = Some(self.head.borrow().unwrap());
            self.head = RefCell::new(Some(tail_index));
            let _ = std::mem::replace(&mut self.cache.borrow().get(tail_index), Some(&Some(node)));

            return;
        }

        // list has space
        let mut cache_mut = self.cache.borrow_mut();
        let position = cache_mut.len() - 1;

        let current_node = cache_mut
            .get_mut(self.head.borrow().unwrap())
            .unwrap()
            .as_mut();

        // new head
        node.prev = None;
        node.next = Some(self.head.borrow().unwrap());

        // let mut cache_mut = self.cache.borrow_mut();
        // let position = cache_mut.len() - 1;

        // current_node..unwrap().prev = Some(position);

        // previous head should have previous set to new head index
        if let Some(a) = current_node {
            a.prev = Some(position);
        };

        cache_mut.push(Some(node));

        // let b_i = self.head.borrow_mut();
        self.head = RefCell::new(Some(position + 1));

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

        for val in cache.cache.into_inner() {
            println!("{:?}", val);
        }

        println!("HEAD {:?}", cache.head);
        println!("TAIL {:?}", cache.tail);

        // cache.get(&3);
    }
}
