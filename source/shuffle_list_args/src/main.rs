use clap::{App, Arg};
use rand::thread_rng;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;
use std::fs;
use std::io;
use std::io::prelude::*;

fn fill_vec(input: &String, v: &mut Vec<String>) {
    let contents = fs::read_to_string(input).expect("Error reading the file");
    for i in contents.lines() {
        v.push(i.to_string());
    }
}

fn shuffle_input() {
    let mut v: Vec<String> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => {
                v.push(l);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    let mut rng = thread_rng();
    let mut irs = Irs::default();
    irs.shuffle(&mut v, &mut rng).expect("on shuffle");
    for i in &v {
        println!("{}", i);
    }
}

struct Arguments {
    files: Vec<String>,
}

fn parse_args() -> Arguments {
    let matches = App::new("shuffle_args")
        .help("shuffle text")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("input file(s)")
                .multiple(true)
                .default_value("<STDIN>")
                .allow_invalid_utf8(true),
        )
        .get_matches();
    let f = matches.values_of_lossy("files").unwrap();
    Arguments { files: f }
}

fn main() {
    let mut v: Vec<String> = Vec::new();
    let args = parse_args();
    if args.files.len() == 1 && args.files[0] == "<STDIN>" {
        shuffle_input();
    } else if args.files.len() > 0 {
        for i in &args.files {
            fill_vec(i, &mut v);
        }
        let mut rng = thread_rng();
        let mut irs = Irs::default();
        irs.shuffle(&mut v, &mut rng).expect("on shuffle");
        for i in &v {
            println!("{}", i);
        }
    }
}
