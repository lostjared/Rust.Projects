use std::io::Read;
use std::io::Write;

async fn proc_socket(mut s: std::net::TcpStream) {
    let mut buf = [0u8; 1024];
    s.read(&mut buf).unwrap();
    s.write_all(&buf).expect("on write");
    println!("wrote: {}", std::str::from_utf8(&buf).unwrap());
}

#[async_std::main]
async fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:8003").unwrap();
    for s in listener.incoming() {
        let s = s.unwrap();
        proc_socket(s).await;
    }
}