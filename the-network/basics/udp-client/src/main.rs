// UDP client found on page 11
use std::process;
use std::net::UdpSocket;

// lmaooo make sure that you set up a listener on UDP port 4444 when testing this
static TARGET_HOST: &str = "0.0.0.0:4444";

fn main() {
    let socket = match UdpSocket::bind("127.0.0.1:34545") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("could not bind to 127.0.0.1: {}", e);
            process::exit(1);
        }
    };
    socket.connect(TARGET_HOST).expect(format!("could not connect to {}", TARGET_HOST).as_ref());

    match socket.send(b"AAABBBCCC") {
        Ok(_bytes_written) => (),
        Err(e) => {
            eprintln!("could not write data over socket: {}", e);
            process::exit(2);
        }
    }

    let mut buffer: [u8; 4096] = [0; 4096]; // initializes u8 (byte array) with 4096 zeros
    match socket.recv(&mut buffer) {
        Ok(_bytes_read) => println!("{}", String::from_utf8_lossy(&buffer)),
        Err(e) => {
            eprintln!("could not read data over UDP socket: {}", e);
            process::exit(3);
        }
    }
}
