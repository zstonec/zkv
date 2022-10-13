use protocol::pb::commands::{CommandResponse, Value, Del, Get, Set};
use crate::{CommandService, Storage};

impl CommandService for Set {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.set(self.key, self.value.unwrap_or_default(), self.ttl) {
            Ok(Some(value)) => value.into(),
            Ok(None) => Value::default().into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Get {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.key) {
            Ok(Some(value)) => value.into(),
            Ok(None) => Value::default().into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Del {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.del(self.keys) {
            Ok(value) => Value::from(value as i64).into(),
            Err(e) => e.into(),
        }
    }
}