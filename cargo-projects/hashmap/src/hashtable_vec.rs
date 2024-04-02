use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

const DEFAULT_BUCKETS_NUM: usize = 1;

/// HashTableVec
///
/// [b] - these are buckets (a vector)
/// [i] - these are items (a vector)
///
/// [b1] -> [i1] -> [i2] -> [i3]
/// [b2] -> [i1] -> [i2] -> [i3]
/// [b3] -> [i1] -> [i2] -> [i3]
///
#[derive(Debug, Default)]
pub struct HashtableVec<Key, Value> {
    buckets: Vec<Bucket<Key, Value>>,
    items: usize,
    capacity: usize,
}

#[derive(Debug, Clone)]
struct Bucket<Key, Value> {
    items: Vec<(Key, Value)>,
}

impl<Key: Debug, Value: Debug> HashtableVec<Key, Value> {
    pub fn new() -> Self {
        HashtableVec {
            buckets: vec![],
            items: 0,
            capacity: DEFAULT_BUCKETS_NUM,
        }
    }
}

impl<Key: Debug + Copy, Value: Debug + Copy> HashtableVec<Key, Value> {
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

    pub fn get(&mut self, key: Key) -> Value {
        let bucket_index = self.hash_key(key);
        let value = self.buckets[bucket_index].items[0];

        value.1
    }

    fn bucket_index(&mut self, key: Key) -> usize {
        // Initially there will be no buckets
        if self.buckets.is_empty() {
            self.buckets.push(Bucket { items: vec![] });
            return self.hash_key(key);
        }

        if self.items == self.capacity {
            self.allocate();
        }

        self.hash_key(key)
    }

    fn hash_key(&mut self, key: Key) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        (hasher.finish() % self.capacity as u64) as usize
    }

    fn allocate(&mut self) {
        if self.capacity == self.items {
            let new_capacity = self.capacity + 16;
            let mut new_vec = HashtableVec::with_capacity(new_capacity);

            self.capacity = new_capacity;

            for index in 0..self.items {
                let bucket = new_vec.buckets.get_mut(index);

                if bucket.is_none() {
                    new_vec.buckets.push(self.buckets[index].clone());
                    continue;
                }

                // we need to rehash the key since capacity has changed
                let h = self.hash_key(self.buckets[index].items[0].0);

                new_vec.buckets[h] = self.buckets[index].clone();
            }

            self.buckets = new_vec.buckets;
            // self.capacity = new_vec.capacity;
        }
    }
}

#[cfg(test)]
mod hashtable_tests {
    use super::HashtableVec;

    #[test]
    fn set_and_get_a_single_value() {
        let mut ht = HashtableVec::new();

        ht.set("key", "value");
        let actual = ht.get("key");

        assert!(actual == "value");
        assert!(ht.capacity == 1);
        assert!(ht.items == 1);
    }

    #[test]
    fn set_and_get_multiple_values() {
        let mut ht = HashtableVec::new();

        // ht length is 1
        ht.set("key", "value");
        // ht length is 17 at this point
        ht.set("key2", "value2");
        ht.set("key3", "value3");
        ht.set("key4", "value4");

        assert!(ht.get("key") == "value");
        assert!(ht.get("key2") == "value2");
        assert!(ht.get("key3") == "value3");
        assert!(ht.get("key4") == "value4");
        assert!(ht.capacity == 17);
        assert!(ht.items == 4);
    }

    #[test]
    fn with_capacity_sets_capacity() {
        let mut ht = HashtableVec::with_capacity(2);

        // Initial capacity is 2
        ht.set("key", "value");
        ht.set("key2", "value2");
        // Capacity changes to 2 + 16
        ht.set("key3", "value3");

        assert!(ht.get("key") == "value");
        assert!(ht.get("key2") == "value2");
        assert!(ht.get("key3") == "value3");
        assert!(ht.items == 3);
        assert!(ht.capacity == 18);
    }

    // #[test]
    // fn hashtable_allocates_more_space_when_out_of_space() {
    //     let mut ht = HashtableVec::new();

    //     ht,set("key", "value");

    //     assert!(ht.
}
