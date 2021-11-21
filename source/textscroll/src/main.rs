use std::env;
use std::fs;
use std::io::{self, Write};
use std::{thread, time};


fn scrolltext(text: &String, timeout: u64) {
    let millis = time::Duration::from_millis(timeout);
    for i in text.chars() {
        print!("{}", i);
        io::stdout().flush().unwrap();
        thread::sleep(millis);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let val1 = args.get(1).unwrap();
        let val2 = args.get(2).unwrap();
        let contents = fs::read_to_string(val1).expect("Error reading the file");
        let num: u64 = val2.to_string().parse().unwrap();
        scrolltext(&contents, num);
    }
    else {
        println!("Error: input_file output_file");
    }
}
