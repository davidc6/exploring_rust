use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

const BUCKETS: usize = 16;

#[derive(Debug, Clone)]
struct Bucket<Key, Value> {
    items: Vec<(Key, Value)>,
}

#[derive(Debug)]
pub struct HashTable<Key, Value, const DEFAULT_NUMBER_OF_BUCKETS: usize = BUCKETS> {
    buckets: Vec<Bucket<Key, Value>>,
    items: usize,
}

impl<Key: Hash + Debug + Clone, Value: Debug + Clone> Default for HashTable<Key, Value> {
    fn default() -> Self {
        Self::new()
    }
}

// Only put bound on implementation
impl<Key: Hash + Debug + Clone, Value: Debug + Clone> HashTable<Key, Value> {
    pub fn new() -> Self {
        HashTable {
            buckets: vec![Bucket { items: vec![] }; BUCKETS],
            items: 0,
        }
    }
}

impl<Key: Hash + Debug + Copy + Clone, Value: Debug + Clone> HashTable<Key, Value> {
    fn bucket_index(&mut self, key: Key) -> usize {
        // if self.buckets.is_empty() {
        // resize the underlying vector
        // }

        if self.buckets.len() == self.items {
            self.allocate();
        }

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        (hasher.finish() % self.buckets.len() as u64) as usize
    }

    pub fn set(&mut self, key: Key, value: Value) -> Option<Value> {
        // TODO: resize the hashmap
        // if self.buckets.is_empty() {
        //     let mut b = Vec::with_capacity(1);
        //     b.extend((0..1).map(|_| Vec::new()));
        // }

        // hash the value and store the key
        // let mut hasher = DefaultHasher::new();
        // key.hash(&mut hasher);

        let bucket_index = self.bucket_index(key);

        if let Some(bucket) = self.buckets.get_mut(bucket_index) {
            bucket.items.push((key, value))
        }

        self.items += 1;

        None
    }

    pub fn length(&self) -> usize {
        self.items
    }

    fn allocate(&mut self) {
        // for _ in 0..DEFAULT_NUMBER_OF_BUCKETS {
        //     self.buckets.push(Bucket { items: vec![] })
        // }

        // self

        // if current length is at capacity and we are inserting a new item then we need to r
        if self.buckets.len() == self.items {
            let new_capacity = self.items * 2; // double
                                               // let mut new_buckets = vec![Bucket { items: vec![] }; new_capacity];

            for index in self.items..new_capacity {
                self.buckets[index] = Bucket { items: vec![] }
            }

            // for (index, bucket) in self.buckets.iter_mut().enumerate() {
            // new_buckets[index] = bucket;
            // }

            // self.buckets = new_buckets;
        }

        // self
    }

    pub fn get(&mut self, key: Key) -> Option<&Value> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let bucket_index = (hasher.finish() % self.buckets.len() as u64) as usize;

        if let Some(item) = self.buckets.get(bucket_index) {
            Some(&item.items[0].1)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HashTable;

    #[test]
    fn can_add_to_and_get_from_hashtable() {
        let mut hash_table = HashTable::<_, _, 16>::new();

        hash_table.set("key", "value");

        assert_eq!(hash_table.get("key"), Some(&"value"));
    }

    #[test]
    fn allocate_16_buckets_on_initialisation_by_default() {
        let hash_table: HashTable<_, _> = HashTable::<&str, &str>::new();
        let buckets_len = hash_table.buckets.len();

        assert!(buckets_len == 16);
    }

    #[test]
    fn get_length_of_hashtable() {
        let mut hash_table = HashTable::<&str, &str>::new();

        assert!(hash_table.length() == 0);

        hash_table.set("Hello", "World");

        assert!(hash_table.length() == 1);
    }
}
