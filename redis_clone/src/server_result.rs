use std::fmt;

use crate::resp::RESP;

#[derive(Debug, PartialEq)]
pub enum ServerError {
    CommandError,
    IncorrectData, 
    StorageNotInitialized,
}

#[derive(Debug)]
pub enum ServerMessage {
    Data(ServerValue),
    Error(ServerError),
}

#[derive(Debug)]
pub enum ServerValue {
    RESP(RESP),
}



impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::CommandError => write!(f, "Error while processing"),
             ServerError::IncorrectData => {
                writeln!(f, "Data received from stream is incorrect")
             },
             ServerError::StorageNotInitialized => {
                writeln!(f, "Storage has not been initialized")
             },
        }
    }
}

pub type ServerResult = Result<ServerValue, ServerError>;
