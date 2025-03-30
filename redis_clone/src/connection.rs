use core::fmt;

use crate::{request::Request, server_result::ServerError};

#[derive(Debug)]
pub enum ConnectionMessage {
    Request(Request),
}

#[derive(Debug)]
pub enum ConnectionError {
    ServerError(ServerError),
}


impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionError::ServerError(e) => {
                writeln!(f, "{}", format!("Server error: {}", e))
            }
            
        }
    }
    
}