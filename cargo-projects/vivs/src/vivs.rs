use std::{collections::HashMap, hash::Hash};

pub trait DataStoreKey: PartialEq + Eq + Hash {} // "subtrait" of PartialEq, Eq and Hash traits
impl<T: PartialEq + Eq + Hash> DataStoreKey for T {} // blanket implementation

pub struct DataStore<K: DataStoreKey, V> {
    map: HashMap<K, V>
}

impl<K: DataStoreKey, V> Default for DataStore<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: DataStoreKey, V> DataStore<K, V> {
    pub fn new() -> DataStore<K, V> {
        DataStore {
            map: HashMap::new()
        }
    }

    pub fn set(&mut self, key: K, value: V) -> Option<V> {
        self.map.insert(key, value)
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.map.get(&key)
    }

    pub fn delete(&mut self, key: K) -> Option<V> {
        self.map.remove(&key)
    }

    pub fn count(&self) -> Option<usize> {
        if self.map.is_empty() {
            return None;
        }
        Some(self.map.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_values() {
        let mut store = DataStore::new();

        store.set("first_key".to_owned(), "first_value".to_owned());
        store.set("second_key".to_owned(), "second_value".to_owned());

        assert_eq!(store.get("first_key".to_owned()), Some(&"first_value".to_owned()));
    }

    #[test]
    fn access_nonexistent_value() {
        let mut store = DataStore::new();

        store.set("first_key".to_owned(), "first_value".to_owned());
        assert_eq!(store.get("second_key".to_owned()), None);
    }

    #[test]
    fn removes_values() {
        let mut store = DataStore::new();

        store.set(1, "first_value".to_owned());
        store.set(2, "second_value".to_owned());

        assert_eq!(store.count(), Some(2));

        assert_eq!(store.delete(1), Some("first_value".to_owned()));
        assert_eq!(store.count(), Some(1));

        assert_eq!(store.delete(2), Some("second_value".to_owned()));
        assert_eq!(store.count(), None);
    }
}
