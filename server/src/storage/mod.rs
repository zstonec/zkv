
pub mod memory;

use crate::{KvError, KvPair, Value};

pub trait Storage {
    fn get(&self, key: &str) -> Result<Option<Value>, KvError>;
    fn set(&self, key: String, value: Value, ttl: Option<u32>) -> Result<Option<Value>, KvError>;
    fn del(&self, keys: Vec<String>) -> Result<u32, KvError>;
}