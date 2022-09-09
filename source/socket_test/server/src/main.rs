
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{prelude::*, BufRead};

fn handle(mut sock: TcpStream) -> std::io::Result<()> {
    let r = std::io::BufReader::new(&mut sock);
    let mut re = String::new();
    for i in r.lines() {
        match i {
            Ok(line) => {
                re.push_str(&line);
                re.push('\n');
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    sock.write_all(re.as_bytes()).unwrap();
    println!("Got: {}", re);
    Ok(())
}


fn main() -> std::io::Result<()> {
    let listen = TcpListener::bind("127.0.0.1:8003").unwrap();
    for stream in listen.incoming() {
        let s = stream.unwrap();
        handle(s)?;
    }
    Ok(())
}

