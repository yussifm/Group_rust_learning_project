use tokio::sync::mpsc;

use crate::{resp::RESP, server_result::ServerMessage};



#[derive(Debug)]
pub struct Request{
    pub value: RESP, 
    pub sender: mpsc::Sender<ServerMessage>,
}