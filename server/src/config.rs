
use crate::KvError;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralConfig {
    pub addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogConfig {
    pub path: String,
    pub rotation: RotationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RotationConfig {
    Hourly,
    Daily,
    Never,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "args")]
pub enum StorageConfig {
    MemTable,
    SledDb(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerConfig {
    pub general: GeneralConfig,
    pub storage: StorageConfig,
    pub log: LogConfig,
}

impl ServerConfig {
    pub fn load(path: &str) -> Result<Self, KvError> {
        let config = fs::read_to_string(path)?;
        let config = toml::from_str(&config)?;
        Ok(config)
    }
}