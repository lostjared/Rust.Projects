
use std::net::TcpStream;
use std::io::{prelude::*};
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut s = TcpStream::connect("127.0.0.1:8003").unwrap();
    let value = String::from("Hello");
    s.write_all(value.as_bytes()).unwrap();
    s.flush()?;
    let mut data = [0u8; 5];
    s.read(&mut data)?;
    let sval = std::str::from_utf8(&data).unwrap();
    println!("Got back: {}", sval);
    Ok(())

}