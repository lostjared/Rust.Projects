
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let values = Arc::new(Mutex::new(vec![0, 1, 2]));
    let mut handles = vec![];
    for _i in 0..10 {
        let v = values.clone();
        handles.push(thread::spawn(move || {
            for i in 0..100 {
                let mut val = v.lock().unwrap();
                val.push(i);
            }
        }));
    }
    for i in handles {
        i.join().unwrap();
    }

    let v = values.lock().unwrap();
    for i in v.iter() {
        println!("{}", i);
    }
}
