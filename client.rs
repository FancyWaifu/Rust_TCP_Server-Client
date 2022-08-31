use std::io::prelude::*;
use std::net::TcpStream;
use std::str::Bytes;

fn main() {

    let mut stream: TcpStream;

    while 1 == 1
    {
        stream = TcpStream::connect("127.0.0.1:7878").unwrap();

        let message: String = input("Message: ");

        handle_connection(stream, message);
    }
}

fn handle_connection(mut connection: TcpStream, message: String)
{
    let mut buffer: [u8; 1024] = [0; 1024]; 

    let mess_bytes: Bytes = message.bytes();

    let mut nice_vec: Vec<u8> = Vec::new(); 

    for x in mess_bytes
    {
        nice_vec.push(x);
    }

    //the u8 part is the ASCII number you want to asign
    //65 = A
    for x in 0..nice_vec.len()
    {
        buffer[x] = nice_vec[x];
    }
    
    connection.write(&mut buffer).unwrap();
}
