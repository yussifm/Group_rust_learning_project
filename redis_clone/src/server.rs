use std::fmt;

use crate::resp::RESP;

#[derive(Debug, PartialEq)]
pub enum ServerError {
    CommandError,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandError => write!(f, "Error while processing"),
        }
    }
}

pub type ServerResult<T> = Result<T, ServerError>;

pub fn process_request(request: RESP) -> ServerResult<RESP> {
    let elements = match request {
        RESP::Array(v) => v,
        _ => {
            return Err(ServerError::CommandError);
        }
    };

    if elements.is_empty() {
        return Err(ServerError::CommandError);
    }

    let mut command = Vec::new();

    for elem in elements.iter() {
        match elem {
            RESP::BulkString(v) => command.push(v),
            _ => {
                return Err(ServerError::CommandError);
            }
        }
    }

    if command.is_empty() {
        return Err(ServerError::CommandError);
    }

    match command[0].to_lowercase().as_str() {
        "ping" => Ok(RESP::SimpleString(String::from("PONG"))),
        "echo" => Ok(RESP::BulkString(command[1].clone())),
        _ => Err(ServerError::CommandError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precess_request_ping() {
        let request = RESP::Array(vec![RESP::BulkString(String::from("PING"))]);
        let output = process_request(request).unwrap();
        assert_eq!(output, RESP::SimpleString(String::from("PONG")));
    }

    #[test]
    fn test_process_request_echo() {
        let request = RESP::Array(vec![
            RESP::BulkString(String::from("ECHO")),
            RESP::BulkString(String::from("42")),
        ]);
        let output = process_request(request).unwrap();
        assert_eq!(output, RESP::BulkString(String::from("42")));
    }
}
