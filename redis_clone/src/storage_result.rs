use std::fmt;

#[derive(Debug)]
pub enum StorageError {
    IncorrectRequest,
    CommandNotAvailable(String),
    CommandSyntaxError(String),
    CommandInternalError(String),
}



impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::IncorrectRequest => {
                write!(f, "The client sent an incorrect request!")
            }
            StorageError::CommandNotAvailable(c) => {
                write!(f, "The requested command {} is not available!", c)
            }
            StorageError::CommandSyntaxError(c)=> {
                write!(f,"Syntax error while processing {}", c)
            },
            StorageError::CommandInternalError(c)=> {
                write!(f,"Internal error while processing {}!", c)
            },
        }
    }
}

pub type StorageResult<T> = Result<T, StorageError>;