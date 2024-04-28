use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 512];
                loop {
                    let read_count = stream.read(&mut buffer).unwrap();
                    if read_count == 0 {
                        break;
                    }
                    let _ = stream.write(b"+PONG\r\n");
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
