use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

/// Expirations implementation options:
///
/// Option 1: key and value in the same store
/// { [key]: { [value]: "hello", [ttl]: "16736377323" } }
/// Potentially not all keys will have expiration set on them,
/// so values that are None will cost more memory.
///
/// [CURRENT] Option 2: 1) key and value Store AND 2) key and expiration Store
/// { [key]: [value] } AND { [key]: [expiry] }
/// We only store keys that have expiration set

#[derive(Clone, Default)]
pub struct DataStore {
    pub db: Arc<RwLock<HashMap<String, String>>>,
    pub expirations: Arc<RwLock<HashMap<String, u64>>>,
}

impl DataStore {
    pub fn new() -> Self {
        Self {
            db: Arc::new(RwLock::new(HashMap::new())),
            expirations: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
