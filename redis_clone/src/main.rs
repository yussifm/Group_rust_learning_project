use resp::{RESP, bytes_to_resp};
use server::process_request;
use std::{sync::{Arc, Mutex}, time::Duration};
use storage::Storage;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

mod resp;
mod resp_result;
mod server;
mod storage;
mod storage_result;
mod set;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let custom_address = "127.0.0.1:6379";
    let listener = TcpListener::bind(custom_address).await?;

    let storage = Arc::new(Mutex::new(Storage::new()));
    println!("Server running on {}", custom_address);

    let mut interval_timer = tokio::time::interval(Duration::from_millis(10));

    loop {
        tokio::select! {
           connection = listener.accept() => {
                match connection {
                    Ok((stream, addr)) => {
                        println!("Connection accepted from: {}", addr);
                        tokio::spawn(handle_connection(stream, storage.clone()));
                    }
                    Err(e) => {
                        println!("Error accepting connection: {}", e);
                        continue;
                    }
                }
            }
            _ = interval_timer.tick() => {
                tokio::spawn(expire_keys(storage.clone()));
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream, storage: Arc<Mutex<Storage>>) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer).await {
            Ok(size) if size != 0 => {
                println!("Received: {:?}", &buffer[..size]);

                let mut index: usize = 0;

                let request = match bytes_to_resp(&buffer[..size].to_vec(), &mut index) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        return;
                    }
                };

                let response = match process_request(request, storage.clone()) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Error parsing command: {}", e);
                        return;
                    }
                };

                // let response = RESP::SimpleString(String::from("PONG"));

                if let Err(e) = stream.write_all(response.to_string().as_bytes()).await {
                    eprintln!("Error writing to socket: {}", e);
                    break;
                }
                if let Err(e) = stream.flush().await {
                    eprintln!("Error flushing socket: {}", e);
                    break;
                }
                println!("Sent response: {}", response);
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
}

async fn expire_keys(storage: Arc<Mutex<Storage>>) {
    let mut guard = storage.lock().unwrap();
    guard.expire_keys();
}
