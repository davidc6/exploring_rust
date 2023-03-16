use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Default)]
pub struct DataStore {
    pub db: HashMap<String, String>,
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore { db: HashMap::new() }
    }
}

#[derive(Clone, Default)]
pub struct DataStoreWrapper {
    pub db: Arc<RwLock<DataStore>>,
}

// Enables wrapping of a hashmap in a RwLock which is a lock that enables multiple readers and one writer at a time,
// which in turn gets wrapped in an Arc - a thread-safe reference counting pointer
impl DataStoreWrapper {
    pub fn new() -> Self {
        Self {
            db: Arc::new(RwLock::new(DataStore::new())),
        }
    }
}
