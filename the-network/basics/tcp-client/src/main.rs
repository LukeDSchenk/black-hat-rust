// TCP client found on page 10.
use std::io::{Read, Write};
use std::process;
use std::net::TcpStream;

static TARGET_HOST: &str = "google.com:80";

fn main() {
    let mut stream = match TcpStream::connect(TARGET_HOST) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("could not connect to {}: {}", TARGET_HOST, e);
            process::exit(1);
        }
    };

    match stream.write(b"GET / HTTP/1.1\r\nHost: google.com\r\n\r\n") {
        Ok(_bytes_written) => (), // do nothing on success
        Err(e) => {
            eprintln!("could not write data over socket: {}", e);
            process::exit(2);
        }
    }

    // note to self, you can't just use read_to_string() here because the socket EOF is never reached
    let mut buffer: [u8; 4096] = [0; 4096]; // initializes u8 (byte array) with 4096 zeros
    match stream.read(&mut buffer) {
        Ok(_bytes_read) => println!("{}", String::from_utf8_lossy(&buffer)),
        Err(e) => {
            eprintln!("could not read data over TCP stream: {}", e);
            process::exit(3);
        }
    }
}
