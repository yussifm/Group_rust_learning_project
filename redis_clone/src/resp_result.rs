use std::fmt;
use std::string::FromUtf8Error;

#[derive(Debug, PartialEq)]
pub enum RESPError {
    WrongType,
    FromUtf8,
    OutOfBounds(usize),
}



impl fmt::Display for RESPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RESPError::FromUtf8 => write!(f, "Cannot convert from UTF-8"),
            RESPError::OutOfBounds(index) => write!(f, "Out of bounds at index {}", index),
            RESPError::WrongType => write!(f, "Wrong prefix for RESP type"),
        }
    }
}

impl From<FromUtf8Error> for RESPError {
    fn from(_err: FromUtf8Error) -> Self {
        Self::FromUtf8
    }
}

pub type RESTResult<T> = Result<T, RESPError>;
