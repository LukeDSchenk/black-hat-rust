// TCP server found on page 12.
use std::io::{Read, Write};
use std::process;
use std::net::{TcpListener, TcpStream};
use std::thread;

static BIND_ADDRESS: &str = "0.0.0.0:9999";

/// Handles an incoming TCP connection.
/// TCP streams are automatically closed when their value is dropped.
/// Isn't Rust great?
/// https://doc.rust-lang.org/std/net/struct.TcpStream.html?search=#method.shutdown
fn handle_client(mut stream: TcpStream) {
    // read data from connection
    let mut buffer: [u8; 1024] = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_bytes_read) => println!("[*] Received: {}", String::from_utf8_lossy(&buffer)),
        Err(e) => eprintln!("[ERROR] could not read data from {}: {}", stream.peer_addr().unwrap(), e),
    }

    // send a packet back over the connection
    match stream.write(b"ACK!\n") {
        Ok(_bytes_written) => (),
        Err(e) => eprintln!("[ERROR] could not write data to {}: {}", stream.peer_addr().unwrap(), e),
    }
} // stream gets closed here

fn main() {
    let listener = match TcpListener::bind(BIND_ADDRESS) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[ERROR] could not bind to {}: {}", BIND_ADDRESS, e);
            process::exit(1);
        }
    };
    println!("[*] Listening on {}...", BIND_ADDRESS);

    // this is a handy alternative to calling TcpListener::accept() in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                println!("[*] Accepted connection from: {}", s.peer_addr().unwrap());
                // spin up a client thread to handle incoming data
                thread::spawn(|| {
                    handle_client(s); // note we are not passing a reference, because we want to transfer ownership to the new thread
                });
            },
            Err(e) => eprintln!("[ERROR] an incoming connection failed: {}", e),
        }
    }
}
