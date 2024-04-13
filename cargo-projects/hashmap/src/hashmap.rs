use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

const DEFAULT_BUCKETS_NUM: usize = 16;

#[derive(Debug, Clone)]
struct Bucket<Key, Value> {
    items: Vec<(Key, Value)>,
}

#[derive(Debug)]
pub struct HashTable<Key, Value, const BUCKETS_NUM: usize = DEFAULT_BUCKETS_NUM> {
    buckets: [Option<Bucket<Key, Value>>; BUCKETS_NUM],
    items: usize,
}

impl<Key: Hash + Debug + Clone + Copy, Value: Debug + Clone + Copy> Default
    for HashTable<Key, Value>
{
    fn default() -> Self {
        Self::new()
    }
}

// Only put bound on implementation
impl<Key: Hash + Debug + Clone + Copy, Value: Debug + Clone + Copy, const COUNT: usize>
    HashTable<Key, Value, COUNT>
{
    // Associated constant (constant associated with a certain type)
    const INITIAL_VALUE: Option<Bucket<Key, Value>> = None;

    pub fn new() -> Self {
        HashTable {
            buckets: [Self::INITIAL_VALUE; COUNT],
            items: 0,
        }
    }
}

impl<Key: Hash + Debug + Copy + Clone, Value: Debug + Clone> HashTable<Key, Value> {
    fn bucket_index(&mut self, key: Key) -> usize {
        // we need to extend the array
        if self.buckets.len() == self.items {
            self.allocate();
        }

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        (hasher.finish() % self.buckets.len() as u64) as usize
    }

    pub fn set(&mut self, key: Key, value: Value) -> Option<Value> {
        let bucket_index = self.bucket_index(key);

        if let Some(bucket) = self.buckets.get_mut(bucket_index) {
            let bucket_mut = bucket.as_mut();

            if let Some(current_bucket_mut) = bucket_mut {
                current_bucket_mut.items.push((key, value))
            } else {
                *bucket = Some(Bucket {
                    items: vec![(key, value)],
                });
            }
        }

        self.items += 1;

        None
    }

    pub fn length(&self) -> usize {
        self.items
    }

    fn allocate(&mut self) {
        // if current length is at capacity and we are inserting a new item then we need to r
        if self.buckets.len() == self.items {
            // const new_capacity: usize = self.items * 2;

            // println!("NEW {:?}", new_capacity);

            // let mut n_h = HashTable::default();
            // n_h.set(1, 2);

            // self.buckets = n_h.buckets;

            // for index in self.items..new_capacity {
            //     println!("INDEX {:?}", index);
            //     self.buckets[index] = Some(Bucket { items: vec![] });
            // n_h[index] =
            // }
        }
    }

    pub fn get(&mut self, key: Key) -> Option<&Value> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let bucket_index = (hasher.finish() % self.buckets.len() as u64) as usize;

        if let Some(bucket) = self.buckets.get(bucket_index) {
            let bucket = bucket.as_ref();
            if let Some(current_bucket) = bucket {
                Some(&current_bucket.items[0].1)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod hashmap_tests {
    use crate::hashmap::{HashTable, DEFAULT_BUCKETS_NUM};

    #[test]
    fn can_add_to_and_get_from_hashtable() {
        let mut hash_table = HashTable::new();
        hash_table.set("key", "value");

        assert_eq!(hash_table.get("key"), Some(&"value"));
    }

    #[test]
    fn allocate_16_buckets_on_initialisation_by_default() {
        let mut hash_table = HashTable::new();
        hash_table.set("key", "value");
        let buckets_len = hash_table.buckets.len();

        assert!(buckets_len == 16);
    }

    #[test]
    fn get_length_of_hashtable() {
        let mut hash_table = HashTable::new();

        assert!(hash_table.length() == 0);

        hash_table.set("Hello", "World");

        assert!(hash_table.length() == 1);
    }

    #[test]
    fn default_initial_capacity_is_16() {
        let hash_table = HashTable::<&str, &str>::default();
        assert!(hash_table.buckets.len() == 16);
    }

    #[test]
    fn initialise_with_capacity_10() {
        let hash_table = HashTable::<&str, &str, 10>::new();
        assert!(hash_table.buckets.len() == 10);
    }

    #[test]
    fn extend_the_capacity() {
        let mut hash_table = HashTable::new();

        for _ in 0..DEFAULT_BUCKETS_NUM + 1 {
            hash_table.set("key", "value");
        }

        assert!(hash_table.buckets.len() == 16);
    }
}
