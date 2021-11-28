// search list of files concurrently

use std::thread;
use std::env;
use std::fs::File;
use std::io::{self,BufRead};

fn search_file_lines(filename: &String, search: String) -> (bool, u64) {
    /*
    let contents = fs::read_to_string(filename).expect("Error reading the file");
    for line in contents.lines() {
        if line.contains(search) {
            return true;
        }   
    }
    false*/
    let f = File::open(filename).expect("could not open file");
    let it = io::BufReader::new(f);
    let mut index = 1;
    for i in it.lines() {
        let line = i.unwrap();
        if line.contains(&search) {
            return (true, index);
        }
        index += 1;
    }
    (false, 0)
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
            let result = search_file_lines(&filename, search);
            println!("{} found: {} line: {}", filename, result.0, result.1);
        });
        handles.push(t);
    }
    for i in handles {
        i.join().unwrap();
    }
}
