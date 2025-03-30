use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::sync::mpsc;

use crate::connection::ConnectionMessage;
use crate::request::Request;
use crate::resp::RESP;
use crate::server_result::{ServerMessage, ServerValue};
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

    pub fn expire_keys(&mut self){
        let storage = match self.storage.as_mut() {
            Some(storage)=> storage, 
            None => return,
            
        };
        storage.expire_keys();
    }
}

pub async fn run_server(mut server: Server, mut crx: mpsc::Receiver<ConnectionMessage>) {
    let mut interval_timer = tokio::time::interval(Duration::from_millis(10));
    loop {
        tokio::select! {
            Some(message) = crx.recv() => {
                match message {
                    ConnectionMessage::Request(request) => {
                        process_request(request, &mut server).await;
                    }

                }
            }
            _ = interval_timer.tick() => {
                server.expire_keys();
            }
        }
    }
}

pub async fn process_request(request: Request, server: &mut Server)  {
    let elements = match &request.value {
        RESP::Array(v) => v,
        _ => {
            panic!()
        }
    };

  

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
    match response {
        Ok(v) => {
            request.sender.send(ServerMessage::Data(ServerValue::RESP(v))).await.unwrap();
        }
        Err(e)=> (),
        
    }

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
