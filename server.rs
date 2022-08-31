use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_ttl(100).expect("Could not set TTL");

    for stream in listener.incoming()
    {
        match stream 
        {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => println!("Connection failed!"),
        }
    }
}

fn handle_connection(mut stream: TcpStream)
{
    let mut buffer: [u8; 1024] = [0; 1024];
    
    stream.read(&mut buffer).unwrap();
    println!("Request<{}>: {}", stream.peer_addr().unwrap(), String::from_utf8_lossy(&buffer[..]));
}
