use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    // read 20 bytes at a time from stream echoing back to stream
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    break;
                }
                let response_string = "response>".as_bytes();
                stream.write(&response_string[0..]).unwrap();
                stream.write(&read[0..n]).unwrap();
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:1488").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}
