use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

mod resp; 
mod resp_result;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let custom_address = "127.0.0.1:6379";
    let listener = TcpListener::bind(custom_address).await?;
    println!("Server running on {}", custom_address);
    
    loop {
        match listener.accept().await {
            Ok((mut stream, addr)) => {
                println!("Connection accepted from: {}", addr);
                tokio::spawn(async move {
                    handle_connection(&mut stream).await;
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
                continue;
            }
        }
    }
}

async fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer).await {
            Ok(size) if size != 0 => {
                println!("Received: {:?}", &buffer[..size]);
                let response = "+PONG\r\n";
                if let Err(e) = stream.write_all(response.as_bytes()).await {
                    eprintln!("Error writing to socket: {}", e);
                    break;
                }
                if let Err(e) = stream.flush().await {
                    eprintln!("Error flushing socket: {}", e);
                    break;
                }
                println!("Sent response: {}", response.trim());
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