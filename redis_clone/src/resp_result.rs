use std::fmt;
use std::num;
use std::string::FromUtf8Error;

#[derive(Debug, PartialEq)]
pub enum RESPError {
    FromUtf8,
    IncorrectLength(RESPLength),
    OutOfBounds(usize),
    ParseInt,
    UnKnown,
    WrongType,
}
pub type  RESPLength = i32;





impl fmt::Display for RESPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RESPError::FromUtf8 => write!(f, "Cannot convert from UTF-8"),
            RESPError::IncorrectLength(length) => write!(f, "Incorrect length {}", length),
            RESPError::OutOfBounds(index) => write!(f, "Out of bounds at index {}", index),
            RESPError::ParseInt => write!(f, "Cannot parse string into integer"),
            RESPError::UnKnown => write!(f, "Unknown format for RESP string"),
            RESPError::WrongType => write!(f, "Wrong prefix for RESP type"),
        }
    }
}

impl From<FromUtf8Error> for RESPError {
    fn from(_err: FromUtf8Error) -> Self {
        Self::FromUtf8
    }
}

impl From<num::ParseIntError> for RESPError 
{
    fn from(_err: num::ParseIntError) -> Self {
        Self::ParseInt
    }
    
}

pub type RESTResult<T> = Result<T, RESPError>;
