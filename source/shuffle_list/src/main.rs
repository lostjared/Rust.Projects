extern crate rand;
use rand::thread_rng;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;
use std::env;
use std::fs;

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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Error requires argument of list file");
        std::process::exit(-1);
    }
    shuffle_list(&args[1]);
}
