use crossbeam::select;
use crossbeam::channel::unbounded;
use std::thread;

#[derive(Debug)]
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
                match msg.unwrap() {
                    Message::Message2 => {
                        println!("Quit Message sent exiting... ");
                        std::process::exit(0);
                    }
                    Message::Message1 => {
                        println!("Message 1 sent");
                    }
                }
           },
        }
    }
}
