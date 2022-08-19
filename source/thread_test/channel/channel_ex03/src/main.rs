#[macro_use]
extern crate crossbeam;

use crossbeam::channel::unbounded;
use std::thread;

#[derive(Debug, PartialEq)]
enum Message {
    Message1,
    Message2,
}

fn main() {
    let (channel_send, channel_recv) = unbounded();
    thread::spawn(move || {
        for _ in 0..5 {
            channel_send.send(Message::Message1).unwrap();
        }
        channel_send.send(Message::Message2).unwrap()
    });
    loop {
        select! {
            recv(channel_recv) -> msg => {
                println!("{:?}", msg);
                let val = msg.unwrap();
                if val == Message::Message2 {
                    println!("Quit Message sent exiting... ");
                    std::process::exit(0);
                }
           },
        }
    }
}
