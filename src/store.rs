use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use log::{info};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value {
    pub key: String,
    pub value: String
}

pub struct KvStore {
    store: DashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            store: DashMap::new(),
        }
    }

    pub fn set(&self, value: Value) {
        self.store.insert(value.key.clone(), value.value.clone()); // Insert key and cloned value
        info!("Value set: {:?}", value); // Log value
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match self.store.get(key) {
            Some(v) => {
                let value = v.clone(); // Clone the value to return it
                info!("Value retrieved: {:?}", value); // Log value
                Some(value)
            }
            None => {
                info!("No value found for key: {}", key); // Log not found message
                None
            }
        }
    }

    pub fn remove(&self, key: &str) {
        self.store.remove(key);
        info!("Value removed for key: {}", key); // Log removal
    }
}