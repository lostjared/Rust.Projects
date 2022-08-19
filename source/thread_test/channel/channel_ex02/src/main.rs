use crossbeam::select;
use crossbeam::channel::{unbounded};
use std::thread;

fn main() {
    let (x, y) = unbounded();
    thread::spawn(move || {
        x.send(100).unwrap();
    });
    select! {
        recv(y) -> msg => println!("message {:?}", msg),
    }
}
