use std::path::Path;
use sled::Db;
use protocol::error::KvError;
use protocol::pb::commands::Value;
use crate::Storage;

#[derive(Debug)]
pub struct SledDb(Db);

impl Storage for SledDb {

    fn get(&self, key: &str) -> Result<Option<Value>, KvError> {
        let result = self.0.get(key.as_bytes())?.map(|v| v.as_ref().try_into());
        flip(result)
    }

    fn set(&self, key: String, value: Value, ttl: Option<u32>) -> Result<Option<Value>, KvError> {
        let data: Vec<u8> = value.try_into()?;
        let result = self.0.insert(key, data)?.map(|v| v.as_ref().try_into());
        flip(result)
    }

    fn del(&self, keys: Vec<String>) -> Result<u32, KvError> {
        let mut re: u32 = 0;
        for key in keys {
            if let Some(_) = self.0.remove(key)? {
                re += 1;
            }
        }
        Ok(re)
    }
}

impl SledDb {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(sled::open(path).unwrap())
    }
}

fn flip<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
    x.map_or(Ok(None), |v| v.map(Some))
}