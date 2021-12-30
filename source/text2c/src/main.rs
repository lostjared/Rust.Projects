use std::env;
use std::fs;

fn print_data(inputfile: &String) {
    let contents = fs::read_to_string(inputfile).expect("Error reading the file");
    let val : Vec<&str> = contents.lines().collect();
    println!("unsigned char bytes[] = {{");
    val.iter().for_each(|x| {
        for i in x.as_bytes() {
            print!("{:#04x},", i);
        }
    });
    println!("0x0}};");
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