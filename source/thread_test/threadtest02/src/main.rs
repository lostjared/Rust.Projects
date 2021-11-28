// search list of files concurrently

use std::thread;
use std::env;
use std::fs;

fn process_files(filename: &String, search: &String) -> bool {
    let contents = fs::read_to_string(filename).expect("Error reading the file");
    for line in contents.lines() {
        if line.contains(search) {
            return true;
        }   
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Requires at least two arguments\nsearch file1");
        return;
    }
    let mut handles = vec![];
    for i in 2..args.len() {
        let search: String = String::from(args.get(1).unwrap());
        let filename: String = String::from(args.get(i).unwrap());
        let t = thread::spawn(move || {
            let result = process_files(&filename, &search);
            println!("{} found: {}", filename, result);
        });
        handles.push(t);
    }
    for i in handles {
        i.join().unwrap();
    }
}
