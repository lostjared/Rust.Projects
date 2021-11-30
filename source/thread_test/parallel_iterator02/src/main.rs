// Practice Project using Parallel Iterator
// Search Rust Source Files Concurrently for String
use std::path::Path;
use std::fs;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use rayon::prelude::*;

fn search_file_lines(filename: &Path, search: &String) -> (bool, u64) {
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

fn list_dir(dir: &Path, files: &mut Vec<String>) -> io::Result<()> {
    if dir.is_dir() {
        for e in fs::read_dir(dir)? {
            let e = e?;
            let path = e.path();
            if path.is_dir() {
                list_dir(&path, files)?;
            } else if path.extension() != None && path.extension().unwrap().eq("rs") {
              files.push(path.to_str().unwrap().to_string());
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Search Rust source files\nrequires: search directory");
    }
    let search = args.get(1).unwrap();
    let path = args.get(2).unwrap();
    let mut files : Vec<String> = Vec::new();
    list_dir(Path::new(path), &mut files)?;
    files.into_par_iter().for_each(|filename| {
        let result = search_file_lines(Path::new(&filename), &search);
        if result.0 != false && result.1 != 0 {
            println!("search found: {} at line {}", filename, result.1);
        }
    });
    Ok(())
}
