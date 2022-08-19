
#[macro_use]
extern crate crossbeam;

use crossbeam::channel::unbounded;
use std::thread;

use crate::ConnectivityCheck::*;

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    End,
}

fn main() {
    let n_messages = 5;
    let (re_tx, re_rx) = crossbeam::channel::unbounded();
    let (res_tx, res_rx) = crossbeam::channel::unbounded();
    thread::spawn(move || loop {
        match re_rx.recv().unwrap() {
            Pong => println!("Pong!"),
            Ping => res_tx.send(Pong).unwrap(),
            End => return,
        }
    });
    for _ in 0..n_messages {
        re_tx.send(Ping).unwrap();
    }
    re_tx.send(End).unwrap();
    for _ in 0..n_messages {
        select! {
            recv(res_rx) -> msg => println!("{:?}", msg),
        }
    }
}


