extern crate rand;
use rand::thread_rng;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

fn shuffle_list(input: &str) {
    let contents = fs::read_to_string(input).expect("Error reading the file");
    let mut lines_list: Vec<&str> = contents.lines().collect();
    let mut rng = thread_rng();
    let mut irs = Irs::default();
    irs.shuffle(&mut lines_list, &mut rng).expect("on shuffle");
    for i in &lines_list {
        println!("{}", i);
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        shuffle_input();
    } else {
        shuffle_list(&args[1]);
    }
}
