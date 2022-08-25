
use clap::{App, Arg};
use std::io::{BufRead,Read};

struct Arguments {
    filenames: Vec<String>,
    lines: bool,
    ch: bool,
    words: bool
}

fn parse_args() -> Arguments {

    let matches = App::new("wc_text")
    .version("0.1.0")
    .author("Jared Bruni")
    .help("count lines/bytes")
    .arg(
        Arg::with_name("lines")
            .short('l')
            .long("lines")
            .help("print number of lines")
    )
    .arg(
        Arg::with_name("bytes")
            .short('b')
            .long("bytes")
            .help("output number of bytes")
    )
    .arg(
        Arg::with_name("words")
            .short('w')
            .long("words")
            .help("output number of words")
    )
    .arg(
        Arg::with_name("files")
            .value_name("FILE")
            .help("input file")
            .multiple(true)
            .required(true)
            .allow_invalid_utf8(true),
    )
    .get_matches();
    let f = matches.values_of_lossy("files").unwrap();
    let l = matches.is_present("lines");
    let b = matches.is_present("bytes");
    let w = matches.is_present("words");
    Arguments {
        filenames: f,
        lines: l,
        ch: b,
        words: w,        
    }
}


fn count_lines(filename: &str) -> usize {
    let f = std::fs::File::open(filename).expect("on open");
    let r = std::io::BufReader::new(f);
    let count = r.lines().count();
    count
}

fn count_chars(filename: &str) -> usize {
    let f = std::fs::File::open(filename).expect("on open");
    let mut r = std::io::BufReader::new(f);
    let mut s = String::new();
    r.read_to_string(&mut s).expect("on read");
    s.len()
}

fn remove_chars(input: &str) -> String {
    let mut output = String::new();
    for i in input.chars() {
        match i {
            'a'..='z' | 'A'..='Z' | ' ' | '\n' => {
                output.push(i);
            }
            _ => {

            }
        }
    }
    output
}

fn count_words(input: &str) -> usize {
    let input = std::fs::read_to_string(input).expect("on read");
    let val = remove_chars(&input);
    let values = val.split(" ");
    values.count()
}


fn count_text(args: &Arguments) -> std::io::Result<()> {
    for i in &args.filenames {
        if args.lines {
            let c = count_lines(i);
            println!("\tlines: {}", c);
        }
        if args.ch {
            let c = count_chars(i);
            println!("\tbytes: {}", c);
        }
        if args.words {
            let c = count_words(i);
            println!("\twords; {}", c);
        }
    }
    Ok(())
}



fn main() -> std::io::Result<()> {
    let args = parse_args();
    count_text(&args)?;
    Ok(())
}