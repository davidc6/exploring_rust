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

impl DataStoreWrapper {
    pub fn new() -> Self {
        // wrap a HashMap in a RwLock lock which enables multiple readers but a single writer
        // then wrap the lock in Arc which is a thread-safe reference counting pointer
        // to enable shared ownership between threads
        Self {
            db: Arc::new(RwLock::new(DataStore::new())),
        }
    }
}
