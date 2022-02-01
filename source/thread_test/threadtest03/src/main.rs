use std::path::Path;
use std::fs;

use std::thread;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn search_file_lines(filename: &Path, search: &str) -> (bool, u64) {
    let f = File::open(filename).expect("could not open file");
    let it = io::BufReader::new(f);
    let mut index = 1;
    for i in it.lines() {
        let line = i.unwrap();
        if line.contains(search) {
            return (true, index);
        }
        index += 1;
    }
    (false, 0)
}

fn list_dir(dir: &Path, search: &str) -> io::Result<()> {
    if dir.is_dir() {
        for e in fs::read_dir(dir)? {
            let e = e?;
            let path = e.path();
            if path.is_dir() {
                list_dir(&path, search)?;
            } else if path.extension() != None && path.extension().unwrap().eq("rs") {
                let val = search_file_lines(&path, search);
                if val.0 && val.1 != 0 {
                    println!("file: {} found: {} lines: {}", path.to_str().unwrap(), val.0, val.1);
                }
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("requires search dir1 dir2");
    }
    let mut handles = vec![];
    for i in 2..args.len() {
        let path : String = String::from(args.get(i).unwrap());
        let srch : String = String::from(args.get(1).unwrap());
        handles.push(thread::spawn(move || {
            list_dir(Path::new(&path), &srch).expect("could not list dir");
        }));
    }
    for q in handles {
        q.join().unwrap();
    }
    Ok(())
}
