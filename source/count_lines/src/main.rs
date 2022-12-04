use std::env;
use std::fs;
use colored::Colorize;


fn count_lines(inputfile: &str) -> (usize, usize, usize) {
    let contents = fs::read_to_string(inputfile).expect("Error reading the file");
    let val: Vec<&str> = contents.lines().collect();
    let mut blanks = 0;
    let mut index = 0;
    for i in &val {
        if !i.trim().is_empty() {
            index += 1;
        } else {
            blanks += 1;
        }
    }
    println!(
        "{} {} {} blank lines, lines: {}, total lines: {}",
        inputfile,
        "Contains".blue(),
        blanks,
        index,
        blanks + index
    );
    (index, blanks, val.len())
}

fn count_lines_by_list(inputfile: &str) -> (usize, usize, usize) {
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
    (total_lines, total_blanks, num_lines)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        /*
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
        );*/
        let val = count_lines_by_list(&args[1]);
        println!(
            "lines: {}, Total Blanks: {}, Total Lines: {}",
            val.0, val.1, val.2
        );
    } else {
        println!("Error: input_file_list");
    }
}
