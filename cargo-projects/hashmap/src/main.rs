use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Debug)]
struct Bucket<Key, Value> {
    items: Vec<(Key, Value)>,
}

#[derive(Debug)]
struct HashTable<Key, Value> {
    buckets: Vec<Bucket<Key, Value>>,
}

// Only put bound on implementation
impl<Key: Hash + Debug, Value: Debug> HashTable<Key, Value> {
    pub fn new() -> Self {
        HashTable { buckets: vec![] }.allocate()
    }
}

impl<Key: Hash + Debug, Value: Debug> HashTable<Key, Value> {
    pub fn add(&mut self, key: Key, value: Value) -> Option<Value> {
        // TODO: resize the hashmap
        // if self.buckets.is_empty() {
        //     let mut b = Vec::with_capacity(1);
        //     b.extend((0..1).map(|_| Vec::new()));
        // }

        // hash the value and store the key
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let bucket_index = (hasher.finish() % self.buckets.len() as u64) as usize;

        if let Some(bucket) = self.buckets.get_mut(bucket_index) {
            bucket.items.push((key, value))
        }

        None
    }

    fn allocate(mut self) -> Self {
        for _ in 0..16 {
            self.buckets.push(Bucket { items: vec![] })
        }

        self
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

fn main() {
    let mut a = HashTable::new();

    a.add("hello", "world");
    a.add("hello2", "world2");
    a.add("hello3", "world3");
    a.add("hello4", "world4");

    println!("HashMap {:?}", a.get("hello2"));
}

#[cfg(test)]
mod tests {
    use crate::HashTable;

    #[test]
    fn can_add_to_and_get_from_hashtable() {
        let mut hash_table = HashTable::new();

        hash_table.add("key", "value");

        assert_eq!(hash_table.get("key"), Some(&"value"));
    }

    #[test]
    fn allocate_16_buckets_on_initialisation_by_default() {
        let hash_table: HashTable<_, _> = HashTable::<&str, &str>::new();
        let buckets_len = hash_table.buckets.len();

        assert!(buckets_len == 16);
    }
}
