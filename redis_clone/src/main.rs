use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let custom_address = "127.0.0.1:6379";
    let listern = TcpListener::bind(custom_address).unwrap();
    
    for stream in listern.incoming() {
        match stream {
            Ok(mut stream)=> {
                handle_connection(&mut stream);
            }
            Err(e)=> {
                println!("There was an error {e}");
            }
            
        }
    }
}

fn handle_connection(stream: &mut TcpStream){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    println!("Received: {:?}", buffer);

    let response = "+PONG\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
