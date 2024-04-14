use crate::element_api::{Element, Empty, Filled};
use std::{
    borrow::Borrow,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

const DEFAULT_BUCKETS_NUM: usize = 1;
const DEFAULT_ALLOCATION_SIZE: usize = 16;

/// HashTable
///
/// There is one vector which holds a number of elements (buckets).
/// Each bucket is a vector which holds items in case of collision.
///
/// [b] - these are buckets (essentially a vector of vectors)
/// [i] - these are items (in a bucket)
///
/// [b1] [i1] [i2] [i3]
/// [b2] [i1] [i2] [i3]
/// [b3] [i1] [i2] [i3]
#[derive(Debug, Default)]
pub struct HashTable<Key, Value> {
    pub buckets: Vec<Bucket<Key, Value>>,
    items: usize,
    capacity: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bucket<Key, Value> {
    pub items: Vec<(Key, Value)>,
}

impl<Key: Debug, Value: Debug> HashTable<Key, Value> {
    pub fn new() -> Self {
        HashTable {
            buckets: vec![Bucket { items: vec![] }],
            items: 0,
            capacity: DEFAULT_BUCKETS_NUM,
        }
    }
}

impl<'a, Key, Value> HashTable<Key, Value> {
    pub fn iter(&'a self) -> HashTableIterator<'a, Key, Value> {
        HashTableIterator {
            ht: self,
            bucket_index: 0,
            in_bucket_index: 0,
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
    ///
    /// If we can borrow a Key as &Q and the resulting reference hashes and compares just like Key then &Q is an acceptable key type.
    ///
    /// Q and K Hash and Eq have to be the same. These can be then considered as same type of reference and the thing pointed to,
    /// has the same semantic meaning. If there's Q it can be used as Key. If there's a reference to Key it can be treated as a reference
    /// to a Q, which would be the same.
    ///
    /// Both &Q and &Key, providing that they have the same Hash and Eq, can be both considered as the same type of reference,
    /// and pointed to thing has the same semantic meaning. In a way Q can be used as Key or a reference to a Key can be treated as
    /// a reference to a Q which will be the same.
    //
    // pub fn get(&self, key: &Key) -> Option<&Value>
    pub fn get<Q: Hash + Eq + ?Sized>(&self, key: &Q) -> Option<&Value>
    where
        Key: Borrow<Q>, // i.e. Key can be borrowed as Q (&Q), i.e. a reference to Q we can get ref to Key
    {
        let bucket_index = self.hash_key(key); // key here is borrowed as Q (&Key)
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
        let bucket_index = self.hash_key(&key);

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

    pub fn has<Q: Hash + Eq + ?Sized>(&mut self, key: &Q) -> bool
    where
        Key: Borrow<Q>,
    {
        self.get(key).is_some()
    }

    pub fn length(&self) -> usize {
        self.items
    }

    pub fn is_empty(&self) -> bool {
        self.items == 0
    }

    pub fn element(&mut self, key: Key) -> Element<Key, Value> {
        let hash = self.hash_key(&key) as u64;
        if let Some(value) = self.get(&key) {
            Element::Filled(Filled {
                hash,
                key,
                value: *value,
                ht: &mut self.buckets,
            })
        } else {
            self.items += 1;
            Element::Empty(Empty {
                hash,
                key,
                ht: &mut self.buckets,
            })
        }
    }

    fn bucket_index(&mut self, key: Key) -> usize {
        if self.buckets.is_empty() {
            self.buckets.push(Bucket { items: vec![] });
            return self.hash_key(&key);
        }

        if self.items == self.capacity {
            self.allocate();
        }

        self.hash_key(&key)
    }

    fn hash_key<Q: Hash + Eq + ?Sized>(&self, key: &Q) -> usize
    where
        Key: Borrow<Q>, // guarantees that when Q is hashed it'll be the same as if Key gets hashed
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        (hasher.finish() % self.capacity as u64) as usize
    }

    fn allocate(&mut self) {
        if self.capacity == self.items {
            let new_capacity = self.capacity + DEFAULT_ALLOCATION_SIZE;
            let mut new_vec = HashTable::with_capacity(new_capacity);

            self.capacity = new_capacity;

            for index in 0..self.items {
                let bucket = new_vec.buckets.get_mut(index);

                if bucket.is_none() {
                    new_vec.buckets.push(self.buckets[index].clone());
                    continue;
                }

                // we need to rehash the key since capacity has changed
                let bucket_index = self.hash_key(&self.buckets[index].items[0].0);
                new_vec.buckets[bucket_index] = self.buckets[index].clone();
            }

            self.buckets = new_vec.buckets;
        }
    }
}

// Here 'a (lifetime) is declared as a generic lifetime parameter
// to be used in the body of the struct.
//
// The way to think about this is: HashTableIterator cannot outlive the reference it holds in ht field.
//
// Each value of type HashTableIterator that gets created, gets a fresh lifetime 'a. Any reference that gets
// stored in ht should enclose 'a and 'a must outlast the lifetime of wherever HashTableIterator stored.
pub struct HashTableIterator<'a, Key: 'a, Value: 'a> {
    ht: &'a HashTable<Key, Value>,
    bucket_index: usize,
    in_bucket_index: usize,
}

// Implement Iterator trait on HashTableIterator struct
// in order to iterate over buckets and the underlying values.
// This implementation does not take ownership of the of original collection
// but references values.
impl<'a, Key, Value> Iterator for HashTableIterator<'a, Key, Value> {
    type Item = (&'a Key, &'a Value);

    fn next(&mut self) -> Option<Self::Item> {
        if self.bucket_index >= self.ht.capacity {
            return None;
        }

        let current_bucket = &self.ht.buckets[self.bucket_index];

        if current_bucket.items.is_empty() {
            self.bucket_index += 1;
            self.next()
        } else {
            let b = &current_bucket.items[self.in_bucket_index];
            if current_bucket.items.len() > 1 {
                self.in_bucket_index += 1;
            } else {
                self.in_bucket_index = 0;
                self.bucket_index += 1;
            }

            Some((&b.0, &b.1))
        }
    }
}

// Implement IntoIterator for HashTable
// which would allow to iterate over the collection by reference
impl<'a, Key, Value> IntoIterator for &'a HashTable<Key, Value> {
    // reference to Key and Value, which are tied to the HashTable
    // Items cannot outlive the map and the map needs to keep on living.
    type Item = (&'a Key, &'a Value);
    type IntoIter = HashTableIterator<'a, Key, Value>; // type of iterator we get back

    fn into_iter(self) -> Self::IntoIter {
        HashTableIterator {
            ht: self,
            in_bucket_index: 0,
            bucket_index: 0,
        }
    }
}

///
pub struct HashTableIntoIter<Key, Value> {
    ht: HashTable<Key, Value>,
    bucket: usize,
}

// Implement Iterator on HashTableIntoIter
// which takes ownership of the underlying data
impl<Key, Value> Iterator for HashTableIntoIter<Key, Value> {
    type Item = (Key, Value);

    fn next(&mut self) -> Option<Self::Item> {
        match self.ht.buckets.get_mut(self.bucket) {
            Some(bucket) => {
                if bucket.items.is_empty() {
                    self.bucket += 1;
                    self.next()
                } else {
                    bucket.items.pop()
                }
            }
            None => None,
        }
    }
}

// Implement IntoIterator trait on HashTable struct
// which takes ownership over the original collection.
impl<Key, Value> IntoIterator for HashTable<Key, Value> {
    type Item = (Key, Value);
    type IntoIter = HashTableIntoIter<Key, Value>;

    fn into_iter(self) -> Self::IntoIter {
        HashTableIntoIter {
            ht: self,
            bucket: 0,
        }
    }
}

#[cfg(test)]
mod hashtable_tests {
    use super::{Element, Empty, HashTable, HashTableIterator};
    use crate::hashtable_vec::Filled;
    use std::collections::HashSet;

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

        assert_eq!(ht.get("key"), Some(&"value"));
        assert_eq!(ht.get("key2"), Some(&"value2"));
        assert_eq!(ht.get("key3"), Some(&"value3"));
        assert_eq!(ht.get("key4"), Some(&"value4"));
        assert_eq!(ht.capacity, 17);
        assert_eq!(ht.items, 4);
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
    fn has_key_returns_true_if_key_exists() {
        let mut ht = HashTable::new();

        ht.set("key", "value");

        assert!(ht.has("key"));
    }

    #[test]
    fn is_empty_returns_true_if_no_items_in_hash_table() {
        let ht: HashTable<&str, &str> = HashTable::new();

        assert!(ht.is_empty());
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

    #[test]
    fn hash_table_iter() {
        let mut ht = HashTable::new();

        ht.set("key1", "value1");
        ht.set("key2", "value2");
        ht.set("key3", "value3");

        let mut iter = HashTableIterator {
            ht: &ht,
            in_bucket_index: 0,
            bucket_index: 0,
        };

        assert_eq!(iter.next(), Some((&"key2", &"value2")));
        assert_eq!(iter.next(), Some((&"key1", &"value1")));
        assert_eq!(iter.next(), Some((&"key3", &"value3")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn hash_table_into_iter_by_ref() {
        let mut ht = HashTable::new();
        ht.set("key", "value");
        ht.set("key2", "value2");
        ht.set("key3", "value3");
        ht.set("key4", "value4");

        let mut count = HashSet::new();

        for (&key, &value) in ht.iter() {
            match key {
                "key" => {
                    count.insert(value);
                    assert_eq!(value, "value")
                }
                "key2" => {
                    count.insert(value);
                    assert_eq!(value, "value2")
                }
                "key3" => {
                    count.insert(value);
                    assert_eq!(value, "value3")
                }
                "key4" => {
                    count.insert(value);
                    assert_eq!(value, "value4")
                }
                _ => unreachable!(),
            }
        }

        assert_eq!(ht.length(), 4); // no item has been confused
        assert_eq!(count.len(), 4); // all case has been visited
    }

    #[test]
    fn hash_table_into_iter_by_value() {
        let mut ht = HashTable::new();
        ht.set("key", "value");
        ht.set("key2", "value2");
        ht.set("key3", "value3");
        ht.set("key4", "value4");

        let mut count = HashSet::new();

        // "into_iter()" gets applied by default in the "for in" pattern
        // So actually this is the same as ht.into_iter() which consumes the collection
        for (key, value) in ht {
            match key {
                "key" => {
                    count.insert(value);
                    assert_eq!(value, "value")
                }
                "key2" => {
                    count.insert(value);
                    assert_eq!(value, "value2")
                }
                "key3" => {
                    count.insert(value);
                    assert_eq!(value, "value3")
                }
                "key4" => {
                    count.insert(value);
                    assert_eq!(value, "value4")
                }
                _ => unreachable!(),
            }
        }

        assert_eq!(count.len(), 4);
    }

    #[test]
    fn empty_element_if_no_value_in_hash_table() {
        let mut ht: HashTable<&str, &str> = HashTable::new();
        let actual = ht.element("hello");

        assert!(matches!(actual, Element::Empty(Empty { .. })))
    }

    #[test]
    fn filled_element_if_value_exists_in_hash_table() {
        let mut ht: HashTable<&str, &str> = HashTable::new();
        ht.set("hello", "world");
        let actual = ht.element("hello");

        assert!(matches!(actual, Element::Filled(Filled { .. })))
    }
}
