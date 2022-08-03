
use std::env;
use std::fs;
use rand::thread_rng;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;
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


fn main() {

    let mut v : Vec<String> = Vec::new();
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        shuffle_input();
    } else {
        for i in args.iter().skip(1) {
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
