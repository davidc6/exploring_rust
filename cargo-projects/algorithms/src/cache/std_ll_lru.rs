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
        let element = data_mut.remove(*loc);
        data_mut.push_back(element.unwrap());

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
    use super::LRUCache;

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
}
