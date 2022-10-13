use crate::pb::commands::{CommandRequest, CommandResponse, Del, Get, Set, value};
use crate::pb::commands::command_request::RequestData;
use crate::pb::commands::Value;

pub mod commands;


impl CommandRequest {
    pub fn new_set(key: impl Into<String>, value: Value, ttl: u32) -> CommandRequest {
        Self {
            request_data: Some(RequestData::Set(Set {
                key: key.into(),
                value: Some(value),
                ttl: Some(ttl)
            }))
        }
    }
    pub fn new_get(key: impl Into<String>) -> CommandRequest {
        Self {
            request_data: Some(RequestData::Get(Get{
                key: key.into(),
            }))
        }
    }
    pub fn new_del(keys: Vec<String>) -> CommandRequest {
        Self {
            request_data: Some(RequestData::Del(Del {
                keys
            }))
        }
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(i))
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(value::Value::String(s))
        }
    }
}

impl From<Value> for CommandResponse {
    fn from(v: Value) -> Self {
        Self {
            status: 200,
            values: vec![v],
            ..Default::default()
        }
    }
}