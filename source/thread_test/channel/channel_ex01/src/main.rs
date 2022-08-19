use crossbeam::select;

#[derive(Debug)]
enum ConnCheck {
    Ping,
    Pong,
    End,
}

fn main() {
    let (re_tx, re_rx) = crossbeam::channel::unbounded();
    let (res_tx, res_rx) = crossbeam::channel::unbounded();
    std::thread::spawn(move || loop {
        match re_rx.recv().unwrap() {
            ConnCheck::Pong => println!("Pong!"),
            ConnCheck::Ping => res_tx.send(ConnCheck::Pong).unwrap(),
            ConnCheck::End => return,
        }
    });
    let num_messages = 5;
    for _ in 0..num_messages {
        re_tx.send(ConnCheck::Ping).unwrap();
    }
    re_tx.send(ConnCheck::End).unwrap();
    for _ in 0..num_messages {
        select! {
            recv(res_rx) -> msg => println!("{:?}", msg),
        }
    }
}
