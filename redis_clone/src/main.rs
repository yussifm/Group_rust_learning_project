use connection::{ run_listener, ConnectionError, ConnectionMessage};
use request::Request;
use resp::{RESP, bytes_to_resp};
use server::{process_request, run_server, Server};
use server_result::{ServerMessage, ServerValue};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use storage::Storage;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    select,
    sync::mpsc,
};

mod connection;
mod request;
mod resp;
mod resp_result;
mod server;
mod server_result;
mod set;
mod storage;
mod storage_result;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let custom_address = "127.0.0.1:6379";
    let listener = TcpListener::bind(custom_address).await?;

    let storage = Storage::new();
    let mut server = Server::new();
    server = server.set_storage(storage);


    let (server_sender, server_receiver) = mpsc::channel::<ConnectionMessage>(32);
    println!("Server running on {}", custom_address);
    tokio::spawn(run_server(server, server_receiver));

    run_listener("127.0.0.1".to_string(), 6379, server_sender).await;

    Ok(())
}


async fn expire_keys(storage: Arc<Mutex<Storage>>) {
    let mut guard = storage.lock().unwrap();
    guard.expire_keys();
}
