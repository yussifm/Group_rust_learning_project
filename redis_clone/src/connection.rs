use core::fmt;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    select,
    sync::mpsc,
};

use crate::{request::Request, resp::bytes_to_resp, server_result::{ServerError, ServerMessage, ServerValue}};

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

 async fn handle_connection(mut stream: TcpStream, server_sender: mpsc::Sender<ConnectionMessage>) {
    let mut buffer = [0; 512];

    let (connection_sender, mut connection_receiver) = mpsc::channel::<ServerMessage>(32);

    loop {
        select! {
             result = stream.read(&mut buffer) => {
                match result {
                    Ok(size) if size != 0 => {
                        println!("Received: {:?}", &buffer[..size]);

                        let mut index: usize = 0;
                        let  resp = match bytes_to_resp(&buffer[..size].to_vec(), &mut index) {
                            Ok(v) => v,
                            Err(e) => {
                                eprint!("Error: {}", e);
                                return;
                            }
                        };


                        let request = Request { value: resp, sender: connection_sender.clone(), };

                        match server_sender.send(ConnectionMessage::Request(request)).await {
                            Ok(()) => {},
                            Err(e) => {
                                eprint!("Error sending request: {}", e);
                                return;
                            }
                        }

                        // let response = match process_request(request, storage.clone()) {
                        //     Ok(v) => v,
                        //     Err(e) => {
                        //         eprintln!("Error parsing command: {}", e);
                        //         return;
                        //     }
                        // };

                        // let response = RESP::SimpleString(String::from("PONG"));

                        // if let Err(e) = stream.write_all(response.to_string().as_bytes()).await {
                        //     eprintln!("Error writing to socket: {}", e);
                        //     break;
                        // }
                        // if let Err(e) = stream.flush().await {
                        //     eprintln!("Error flushing socket: {}", e);
                        //     break;
                        // }
                        // println!("Sent response: {}", response);
                    }
                    Ok(_) => {
                        println!("Connection closed");
                        break;
                    }
                    Err(e) => {
                        println!("Error reading from socket: {}", e);
                        break;
                    }
                }

             }
             Some(response) = connection_receiver.recv() => {
                let _ = match response {
                    ServerMessage::Data(ServerValue::RESP(v)) => stream.write_all(v.to_string().as_bytes()).await,
                    ServerMessage::Error(e) => {
                        eprintln!("Error: {}", ConnectionError::ServerError(e));
                        return;
                    }

                };
             }
        }
    }
}


pub async fn run_listener(host: String, port: u16, server_sender: mpsc::Sender<ConnectionMessage>) {
    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    loop {
        tokio::select! {
            connection = listener.accept() => {
                match connection {
                    Ok((stream, _)) => {
                        tokio::spawn(handle_connection(stream, server_sender.clone()));
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        continue;
                    }
                }
            }
        }
    }
}
