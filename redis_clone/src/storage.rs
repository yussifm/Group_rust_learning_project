use crate::resp::RESP;
use crate::storage_result::{StorageError, StorageResult};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum StorageValue {
    String(String),
}

pub struct Storage {
    store: HashMap<String, StorageValue>,
}

impl Storage {
    pub fn new() -> Self {
        let Store: HashMap<String, StorageValue> = HashMap::new();

        Self { store: Store }
    }

    pub fn processs_command(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        match command[0].to_lowercase().as_str() {
            "ping" => self.command_ping(&command),
            "echo" => self.command_echo(&command),
            _ => Err(StorageError::CommandNotAvailable(command[0].clone())),
            
        }
    }

    fn command_ping(&self, command: &Vec<String>) -> StorageResult<RESP> {
        Ok(RESP::SimpleString("PONG".to_string()))
    }
    fn command_echo(&self, command: &Vec<String>) -> StorageResult<RESP> {
        Ok(RESP::BulkString(command[1].clone()))
    }
}

