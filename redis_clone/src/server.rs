use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;

use crate::connection::ConnectionMessage;
use crate::request::Request;
use crate::resp::RESP;
use crate::storage::{self, Storage};
use crate::storage_result::{StorageError, StorageResult};

pub struct Server {
    pub storage: Option<Storage>,
}

impl Server {
    pub fn new() -> Self {
        Self { storage: None }
    }

    pub fn set_storage(mut self, storage: Storage) -> Self {
        self.storage = Some(storage);
        self
    }
}

pub async fn run_server(mut server: Server, mut crx: mpsc::Receiver<ConnectionMessage>) {
    loop {
        tokio::select! {
            Some(message) = crx.recv() => {
                match message {
                    ConnectionMessage::Request(request) => {
                        process_request(request, &mut server).await;
                    }

                }
            }
        }
    }
}

pub async fn process_request(request: Request, server: &mut Server) -> StorageResult<RESP> {
    let elements = match &request.value {
        RESP::Array(v) => v,
        _ => {
            panic!()
        }
    };

    if elements.is_empty() {
        return Err(StorageError::IncorrectRequest);
    }

    let mut command = Vec::new();

    for elem in elements.iter() {
        match elem {
            RESP::BulkString(v) => command.push(v.clone()),
            _ => {
                panic!()
            }
        }
    }

    let storage = match server.storage.as_mut() {
            Some(storage)=> storage,
            None => panic!(),        
    };


    // let mut guard = storage.lock().unwrap();
    let response = storage.processs_command(&command);
    response

    // match command[0].to_lowercase().as_str() {
    //     "ping" => Ok(RESP::SimpleString(String::from("PONG"))),
    //     "echo" => Ok(RESP::BulkString(command[1].clone())),
    //     _ => Err(ServerError::CommandError),
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_precess_request_ping() {
//         let request = RESP::Array(vec![RESP::BulkString(String::from("PING"))]);
//         let storage = Arc::new(Mutex::new(Storage::new()));

//         let output = process_request(request, storage).unwrap();
//         assert_eq!(output, RESP::SimpleString(String::from("PONG")));
//     }

//     #[test]
//     fn test_process_request_echo() {
//         let request = RESP::Array(vec![
//             RESP::BulkString(String::from("ECHO")),
//             RESP::BulkString(String::from("42")),
//         ]);
//         let storage = Arc::new(Mutex::new(Storage::new()));

//         let output = process_request(request, storage).unwrap();
//         assert_eq!(output, RESP::BulkString(String::from("42")));
//     }
// }
