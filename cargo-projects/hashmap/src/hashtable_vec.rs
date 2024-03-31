use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Debug)]
pub struct HashtableVec<Key, Value> {
    buckets: Vec<Bucket<Key, Value>>,
    items: usize,
    capacity: usize,
}

#[derive(Debug, Clone)]
struct Bucket<Key, Value> {
    items: Vec<(Key, Value)>,
}

impl<Key: Debug + Copy, Value: Debug + Copy> HashtableVec<Key, Value> {
    pub fn new() -> Self {
        HashtableVec {
            buckets: vec![Bucket { items: vec![] }],
            items: 0,
            capacity: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        HashtableVec {
            buckets: vec![Bucket { items: vec![] }; capacity],
            capacity,
            items: 0,
        }
    }
}

impl<Key: Debug + Hash + Copy, Value: Debug + Copy> HashtableVec<Key, Value> {
    pub fn set(&mut self, key: Key, value: Value) -> Option<Value> {
        let bucket_index = self.bucket_index(key);
        self.buckets[bucket_index].items.push((key, value));
        self.items += 1;

        None
    }

    fn bucket_index(&mut self, key: Key) -> usize {
        // we need to extend the array
        if self.buckets.len() == self.items {
            self.allocate();
        }

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        (hasher.finish() % self.buckets.len() as u64) as usize
    }

    fn allocate(&mut self) {
        if self.buckets.len() == self.items {
            let new_capacity = self.items + 16;
            let mut new_vec = HashtableVec::with_capacity(new_capacity);

            for (index, item) in new_vec.buckets.iter_mut().enumerate() {
                if index == self.items {
                    break;
                }

                *item = self.buckets[index].clone();
            }

            self.buckets = new_vec.buckets;
        }
    }
}

#[cfg(test)]
mod hashtable_tests {
    use super::HashtableVec;

    #[test]
    fn init_hashtable() {
        let mut ht = HashtableVec::new();

        ht.set("key", "value");
        ht.set("key1", "value1");
        ht.set("4", "4");

        assert!(ht.items == 3);
    }
}
