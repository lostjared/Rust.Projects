use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{prelude::*};

fn handle(mut sock: TcpStream) -> std::io::Result<()> {
    let mut data = [0u8; 5];
    sock.read(&mut data)?;
    let sval = std::str::from_utf8(&data).unwrap();
    sock.write_all(sval.as_bytes()).unwrap();
    sock.flush()?;
    println!("Got: {}", sval);
    Ok(())
}


fn main() -> std::io::Result<()> {
    let listen = TcpListener::bind("127.0.0.1:8003").unwrap();
    for stream in listen.incoming() {
        let s = stream.unwrap();
        std::thread::spawn(move || {
            handle(s).unwrap();
        });
    }
    Ok(())
}

