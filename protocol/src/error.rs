use thiserror::Error;
use crate::pb::commands::CommandResponse;

#[derive(Error, Debug)]
pub enum KvError {
    #[error("Not found for key : {0}")]
    KeyNotFound(String),
    #[error("Not found for key : {0}, field : {1}")]
    FieldNotFound(String, String),
    #[error("Command is invalid : `{0}`")]
    InvalidCommand(String),
    #[error("Frame is larger than max size")]
    FrameError,
    #[error("Failed to encode protobuf message")]
    EncodeError(#[from] prost::EncodeError),
    #[error("Failed to decode protobuf message")]
    DecodeError(#[from] prost::DecodeError),
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
    #[error("sled error")]
    SledError(#[from] sled::Error),
    #[error("Parse config error")]
    ConfigError(#[from] toml::de::Error),
}

impl From<KvError> for CommandResponse {
    fn from(e: KvError) -> Self {
        Self {
            status: 500,
            message: e.to_string(),
            values: vec![],
            pairs: vec![],
        }
    }
}
