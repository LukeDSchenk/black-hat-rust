// Replacing Netcat found on pages 13-20.
use clap::{Arg, App};
use std::io::{Read, Write};
use std::process;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let matches = App::new("Ruscat")
        .version("1.0")
        .author("Luke Schenk <nineofpine@protonmail.com>")
        .about("A simple Netcat style program written in Rust. Inspired by Justin Seitz's \"Black Hat Python\"")
        .arg(Arg::with_name("target host")
            .help("the IP address of the target host")
            .index(1)
            .requires("target port")
            .required(false))
        .arg(Arg::with_name("target port")
            .help("specify a port to connect to on the target host")
            .index(2))
        .arg(Arg::with_name("listen")
            .help("tell Ruscat to set up a listener")
            .short('l')
            .long("listen")
            .value_name("LISTEN"))
}
