mod Response;

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read, Write};

use crate::Response::RespDataType;


fn handle_client(mut stream: TcpStream) {
    // if PING, return PONG
    let mut buf = [0; 512];
    let command = stream.read(&mut buf).unwrap();
    println!("command: {}", command);
    let response = Response::RespSimpleString { content: "PONG".to_string() };
    stream.write(response.get_response().as_bytes()).unwrap();
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
