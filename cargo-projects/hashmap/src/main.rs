use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Debug)]
struct Bucket<Key: Debug, Value: Debug> {
    items: Vec<(Key, Value)>,
}

#[derive(Debug)]
struct HashTable<Key: Debug, Value: Debug> {
    buckets: Vec<Bucket<Key, Value>>,
}

// Only put bound on implementation
impl<Key: Debug, Value: Debug> HashTable<Key, Value> {
    fn new() -> Self {
        HashTable {
            buckets: vec![Bucket { items: vec![] }],
        }
    }
}

impl<Key: Hash + Debug, Value: Debug> HashTable<Key, Value> {
    fn add(&mut self, key: Key, value: Value) -> Option<Value> {
        // TODO: resize the hashmap
        // if self.buckets.is_empty() {
        //     let mut b = Vec::with_capacity(1);
        //     b.extend((0..1).map(|_| Vec::new()));
        // }

        // hash the value and store the key
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let bucket = (hasher.finish() % self.buckets.len() as u64) as usize;

        self.buckets
            .get_mut(bucket)
            .unwrap()
            .items
            .push((key, value));
        None
    }

    fn get(&mut self, key: Key) -> Option<Value> {
        None
    }
}

fn main() {
    let mut a = HashTable::new();
    a.add("hello", "world");
    a.add("hello2", "world2");
    a.add("hello3", "world3");

    println!("HashMap {:?}", a);
}
