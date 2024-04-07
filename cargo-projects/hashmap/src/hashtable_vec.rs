use std::{
    borrow::Borrow,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

const DEFAULT_BUCKETS_NUM: usize = 1;

/// HashTable
///
/// [b] - these are buckets (a vector)
/// [i] - these are items (a vector)
///
/// [b1] -> [i1] -> [i2] -> [i3]
/// [b2] -> [i1] -> [i2] -> [i3]
/// [b3] -> [i1] -> [i2] -> [i3]
///
#[derive(Debug, Default)]
pub struct HashTable<Key, Value> {
    buckets: Vec<Bucket<Key, Value>>,
    items: usize,
    capacity: usize,
}

#[derive(Debug, Clone)]
struct Bucket<Key, Value> {
    items: Vec<(Key, Value)>,
}

impl<Key: Debug, Value: Debug> HashTable<Key, Value> {
    pub fn new() -> Self {
        HashTable {
            buckets: vec![],
            items: 0,
            capacity: DEFAULT_BUCKETS_NUM,
        }
    }
}

impl<Key: Debug + Copy, Value: Debug + Copy> HashTable<Key, Value> {
    pub fn with_capacity(capacity: usize) -> Self {
        HashTable {
            buckets: vec![Bucket { items: vec![] }; capacity],
            capacity,
            items: 0,
        }
    }
}

impl<Key: Debug + Copy + Eq + Hash, Value: Debug + Copy> HashTable<Key, Value> {
    pub fn set(&mut self, key: Key, value: Value) -> Option<Value> {
        let bucket_index = self.bucket_index(key);

        self.buckets[bucket_index].items.push((key, value));
        self.items += 1;

        None
    }

    /// get() is a generic method that has (accepts) a type parameter Q.
    /// It is generic over the underlying (key) data Q which is specified in the signature of the method.
    /// Key borrows as a Q as stated under the constraints K: Borrow<Q>.
    ///
    /// Both Q and and Key implement Hash and Eq that produce identical results.
    /// Q implements Hash and is not necessarily Sized or "questionably" sized.
    ///   - ?Sized essentially means that the type can either be sized or not (only pointed to and known at runtime).
    ///   This can either be a slice or trait object, or an ordinary value.
    /// These are also called type bounds, the type has to meet these trait bounds.
    ///
    /// Key should also implement the Borrow trait with type Q (majority of types already implement it).
    ///
    /// Since the compiler does not know the size of Q, we do it by reference since as the size of it is known.
    pub fn get<Q: Hash + Eq + ?Sized>(&self, key: &Q) -> Option<&Value>
    where
        Key: Borrow<Q>,
    {
        let bucket_index = self.hash_key(key);

        if bucket_index == 0 && self.items == 0 {
            return None;
        }

        self.buckets[bucket_index]
            .items
            .iter()
            .find(|(existing_key, _)| existing_key.borrow() == key)
            .map(|(_, existing_value)| existing_value)
    }

    pub fn delete(&mut self, key: Key) -> Option<Value> {
        let bucket_index = self.hash_key(key);

        if let Some(index) = self.buckets[bucket_index]
            .items
            .iter()
            .position(|(existing_key, _)| existing_key.borrow() == &key)
        {
            self.items -= 1;
            Some(self.buckets[bucket_index].items.swap_remove(index).1)
        } else {
            None
        }
    }

    pub fn has(&mut self, key: Key) -> bool {
        self.get(&key).is_some()
    }

    pub fn length(&self) -> usize {
        self.items
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

    fn hash_key<Q: Hash>(&self, key: Q) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        (hasher.finish() % self.capacity as u64) as usize
    }

    fn allocate(&mut self) {
        if self.capacity == self.items {
            let new_capacity = self.capacity + 16;
            let mut new_vec = HashTable::with_capacity(new_capacity);

            self.capacity = new_capacity;

            for index in 0..self.items {
                let bucket = new_vec.buckets.get_mut(index);

                if bucket.is_none() {
                    new_vec.buckets.push(self.buckets[index].clone());
                    continue;
                }

                // we need to rehash the key since capacity has changed
                let bucket_index = self.hash_key(self.buckets[index].items[0].0);
                new_vec.buckets[bucket_index] = self.buckets[index].clone();
            }

            self.buckets = new_vec.buckets;
        }
    }
}

// WIP
struct HashTableIterator<Key, Value> {
    ht: HashTable<Key, Value>,
    bucket_index: usize,
    bucket_at: usize,
}

impl<'a, Key, Value> Iterator for HashTableIterator<&'a Key, &'a Value> {
    type Item = Bucket<&'a Key, &'a Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ht.buckets.get(self.bucket_index).cloned()
    }
}

#[cfg(test)]
mod hashtable_tests {
    use super::HashTable;

    #[test]
    fn set_and_get_a_str() {
        let mut ht = HashTable::new();

        ht.set("key", "value");

        assert_eq!(ht.get("key"), Some(&"value"));
        assert!(ht.capacity == 1);
        assert!(ht.items == 1);
    }

    #[test]
    fn set_and_get_an_integer() {
        let mut ht = HashTable::new();

        ht.set(1, "value");

        assert_eq!(ht.get(&1), Some(&"value"));
        assert!(ht.capacity == 1);
        assert!(ht.items == 1);
    }

    #[test]
    fn set_and_get_multiple_values() {
        let mut ht = HashTable::new();

        // ht length is 1 (by default)
        ht.set("key", "value");
        // ht length is 17 at this point after the allocation
        ht.set("key2", "value2");
        ht.set("key3", "value3");
        ht.set("key4", "value4");

        assert!(ht.get("key") == Some(&"value"));
        assert!(ht.get("key2") == Some(&"value2"));
        assert!(ht.get("key3") == Some(&"value3"));
        assert!(ht.get("key4") == Some(&"value4"));
        assert!(ht.capacity == 17);
        assert!(ht.items == 4);
    }

    #[test]
    fn with_capacity_sets_capacity() {
        let mut ht = HashTable::with_capacity(2);

        // Initial capacity is 2
        ht.set("key", "value");
        ht.set("key2", "value2");
        // Capacity changes to 2 + 16
        ht.set("key3", "value3");

        assert!(ht.get("key") == Some(&"value"));
        assert!(ht.get("key2") == Some(&"value2"));
        assert!(ht.get("key3") == Some(&"value3"));
        assert!(ht.items == 3);
        assert!(ht.capacity == 18);
    }

    #[test]
    fn set_delete_key() {
        let mut ht = HashTable::new();

        ht.set("key", "value");
        assert!(ht.get("key") == Some(&"value"));
        ht.delete("key");

        assert!(ht.get("key").is_none());
        assert!(ht.items == 0);
        assert!(ht.capacity == 1);
    }

    #[test]
    fn set_delete_key_after_allocation() {
        let mut ht = HashTable::new();

        ht.set("key", "value");
        ht.set("key2", "value2");
        assert!(ht.get("key") == Some(&"value"));

        ht.delete("key");

        assert!(ht.get("key").is_none());
        assert!(ht.items == 1);
        assert!(ht.capacity == 17);
    }

    #[test]
    fn has_returns_true_if_key_exists() {
        let mut ht = HashTable::new();

        ht.set("key", "value");

        assert!(ht.has("key"));
    }

    #[test]
    fn length_returns_length_of_hashmap() {
        let mut ht = HashTable::new();

        ht.set("key", "value");

        assert!(ht.length() == 1);
    }

    #[test]
    fn length_returns_length_of_hashmap_allocated() {
        let mut ht = HashTable::new();

        ht.set("key", "value");
        ht.set("key2", "value2");

        assert!(ht.length() == 2);
    }

    #[test]
    fn length_returns_length_of_hashmap_allocated_and_keys_deleted() {
        let mut ht = HashTable::new();

        ht.set("key", "value");
        ht.set("key2", "value2");
        ht.delete("key");
        ht.delete("key2");

        assert_eq!(ht.length(), 0);
    }
}
