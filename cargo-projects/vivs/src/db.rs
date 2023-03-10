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
        Self {
            db: Arc::new(RwLock::new(DataStore::new())),
        }
    }
}
