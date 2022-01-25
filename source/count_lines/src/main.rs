use std::env;
use std::fs;

fn count_lines(inputfile: &String) -> (usize, usize, usize) {
    let contents = fs::read_to_string(inputfile).expect("Error reading the file");
    let val: Vec<&str> = contents.lines().collect();
    let mut blanks = 0;
    let mut index = 0;
    for i in &val {
        if i.trim().len() > 0 {
            index += 1;
        } else {
            blanks += 1;
        }
    }
    println!(
        "{} contains {} blank lines, lines: {}, total lines: {}",
        inputfile,
        blanks,
        index,
        blanks + index
    );
    (index, blanks, val.len())
}

fn count_lines_by_list(inputfile: &String) -> (usize, usize, usize) {
    let contents = fs::read_to_string(inputfile).expect("Error reading the file");
    let val: Vec<&str> = contents.lines().collect();
    let mut num_lines = 0;
    let mut total_blanks = 0;
    let mut total_lines = 0;   
    for i in val {
        let rt_val = count_lines(&i.to_string());
        total_lines += rt_val.0;
        total_blanks += rt_val.1;
        num_lines += rt_val.2;
    }
    (total_lines,total_blanks,num_lines)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut num_lines = 0;
        let mut total_blanks = 0;
        let mut total_lines = 0;
        for i in args.iter().skip(1) {
            let rt_val = count_lines(i);
            total_lines += rt_val.0;
            total_blanks += rt_val.1;
            num_lines += rt_val.2;
        }
        println!(
            "total lines: {}, total blanks: {}, total non-blank: {}",
            num_lines, total_blanks, total_lines
        );
    } else {
        println!("Error: input_file_list");
    }
}
