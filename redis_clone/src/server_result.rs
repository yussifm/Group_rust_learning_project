use std::fmt;

use crate::resp::RESP;

#[derive(Debug, PartialEq)]
pub enum ServerError {
    CommandError,
}

#[derive(Debug)]
pub enum ServerMessage {
    Data(RESP),
    Error(ServerError),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandError => write!(f, "Error while processing"),
        }
    }
}

pub type ServerResult<T> = Result<T, ServerError>;
