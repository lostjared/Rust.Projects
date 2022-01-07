use std::env;
use std::fs;

fn count_lines(inputfile: &String) -> usize {
    let contents = fs::read_to_string(inputfile).expect("Error reading the file");
    let val : Vec<&str> = contents.lines().collect();
    //println!("{} contains {} lines", inputfile, val.len());
    val.len()
}

fn count_blanks(inputfile: &String) -> (usize, usize, usize) {
    let contents = fs::read_to_string(inputfile).expect("Error reading the file");
    let val : Vec<&str> = contents.lines().collect();
    let mut blanks = 0;
    let mut index = 0;
    for i in &val {
        if i.trim().len() > 0 {
            index += 1;
        } else {
            blanks += 1;
        }
    }
    println!("{} contains {} blank lines, lines: {}, total lines: {}", inputfile, blanks, index, blanks+index);
    (index, blanks, val.len())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut num_lines = 0;
        let mut total_blanks = 0;
        let mut total_lines = 0;
        for i in args.iter().skip(1) {
            num_lines += count_lines(i);
            let rt_val = count_blanks(i);
            total_lines += rt_val.0;
            total_blanks += rt_val.1;
        }
        println!("total lines: {}, total blanks: {}, total non-blank: {}", num_lines, total_blanks, total_lines);
    }
    else {
        println!("Error: input_file output_file");
    }
}
