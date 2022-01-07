use std::env;
use std::fs;

fn count_lines(inputfile: &String) -> usize {
    let contents = fs::read_to_string(inputfile).expect("Error reading the file");
    let val : Vec<&str> = contents.lines().collect();
    println!("{} contains {} lines", inputfile, val.len());
    val.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut num_lines = 0;
        for i in args.iter().skip(1) {
            num_lines += count_lines(i);
        }
        println!("total lines: {}", num_lines);
    }
    else {
        println!("Error: input_file output_file");
    }
}
