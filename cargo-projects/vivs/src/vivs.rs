use std::{collections::HashMap, hash::Hash};

pub struct DataStore<K: PartialEq + Eq + Hash, V> {
    map: HashMap<K, V>
}

impl<K: PartialEq + Eq + Hash, V> DataStore<K, V> {
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
}