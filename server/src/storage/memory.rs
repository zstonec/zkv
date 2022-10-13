use dashmap::DashMap;
use protocol::pb::commands::Value;
use crate::{KvError, Storage};

#[derive(Clone, Debug, Default)]
pub struct MemTable {
    table: DashMap<String, Value>
}

impl Storage for MemTable {

    fn get(&self, key: &str) -> Result<Option<Value>, KvError> {
        Ok(self.table.get(key).map(|v|v.value().clone()))
    }

    fn set(&self, key: String, value: Value, ttl: Option<u32>) -> Result<Option<Value>, KvError> {
        Ok(self.table.insert(key, value))
    }

    fn del(&self, keys: Vec<String>) -> Result<u32, KvError> {
        let mut re:u32 = 0;
        for k in keys {
            if let Some(v) = self.table.remove(&k) {
                re += 1;
            }
        }
        Ok(re)
    }
}

impl MemTable {
    pub fn new() -> Self {
        Self::default()
    }
}