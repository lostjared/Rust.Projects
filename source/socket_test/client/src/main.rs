
use std::net::TcpStream;
use std::io::{prelude::*};
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut s = TcpStream::connect("127.0.0.1:8003").unwrap();
    let value = String::from("Hello, World\n");
    s.write_all(value.as_bytes()).unwrap();
    Ok(())

}