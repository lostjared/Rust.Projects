use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn print_data(inputfile: &String) -> io::Result<()> {
    let mut file = File::open(inputfile)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    println!("unsigned char bytes[] = {{");
    for i in buffer {
        print!("{:#04x},", i);
    }
    println!("0x0}};");
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let val1 = args.get(1).unwrap();
        print_data(val1);
    }
    else {
        println!("Error: input_file");
    }
}