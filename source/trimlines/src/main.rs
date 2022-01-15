use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

fn trimlines(inputfile: &String, outputfile: &String) {
    let contents = fs::read_to_string(inputfile).expect("Error reading the file");
    let val: Vec<&str> = contents.lines().collect();
    let mut cfile = File::create(outputfile).expect("Error creating file");
    val.iter().for_each(|x| {
        if x.len() > 0 {
            writeln!(&mut cfile, "{}", x).expect("error on write");
        }
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let val1 = args.get(1).unwrap();
        let val2 = args.get(2).unwrap();
        trimlines(val1, val2);
    } else {
        println!("Error: input_file output_file");
    }
}
