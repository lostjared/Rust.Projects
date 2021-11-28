
use std::thread;

fn main() {
    let mut handles = vec![];
    for i in 1..=10 {
        let t = thread::spawn(move || {
            for z in 1..100 {
                println!("thread: {} - {}", i, z);
            }
        });
        handles.push(t);
    }
    for q in handles {
        q.join().unwrap();
    }
}
