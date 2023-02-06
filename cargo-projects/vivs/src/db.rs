use std::{collections::HashMap, sync::{Arc, RwLock}};

struct DataStore {
    db: HashMap<String, String>
}

pub struct DataStoreWrapper {
    db: std::sync::Arc<std::sync::RwLock<DataStore>>
}

impl DataStoreWrapper {
    pub fn new() -> Self {
        Self { db: Arc::new(RwLock::new(DataStore { db: HashMap::new() })) }
    }
}
