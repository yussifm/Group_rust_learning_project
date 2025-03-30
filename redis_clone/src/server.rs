use std::sync::{Arc, Mutex};

use crate::request::Request;
use crate::resp::RESP;
use crate::storage::{self, Storage};
use crate::storage_result::{StorageError, StorageResult};


pub fn process_request(request: Request, storage: Arc<Mutex<Storage>>) -> StorageResult<RESP> {
    let elements = match request.value {
        RESP::Array(v) => v,
        _ => {
            return Err(StorageError::IncorrectRequest);
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
                return Err(StorageError::IncorrectRequest);
            }
        }
    }

    let mut guard = storage.lock().unwrap();
    let response = guard.processs_command(&command);
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
