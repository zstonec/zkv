use std::sync::Arc;
use protocol::pb::commands::{CommandRequest, CommandResponse};
use protocol::pb::commands::command_request::RequestData;
use crate::memory::MemTable;
use crate::{KvError, Storage};

mod command_service;


/// 对 Command 的处理的抽象
pub trait CommandService {
    /// 处理 Command，返回 Response
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

pub struct Service<Store = MemTable> {
    inner: Arc<Store>,
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner)
        }
    }
}


impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(store)
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        dispatch(cmd, self.inner.as_ref())
    }
}

pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Set(param)) => param.execute(store),
        Some(RequestData::Get(param)) => param.execute(store),
        Some(RequestData::Del(param)) => param.execute(store),
        None => KvError::InvalidCommand("Request has no data".into()).into(),
    }
}
