use clap::{App, Arg};
use std::io::{BufRead,Read};

#[derive(Debug)]
struct Arguments {
    filename: String,
    lines: usize,
    bytes: usize,
    line_number: bool
}

fn parse_args() -> Arguments {
    let matches = App::new("start_text")
        .version("0.1.0")
        .author("Jared Bruni")
        .help("Print starting text")
        .arg(
            Arg::with_name("lines")
                .short('l')
                .long("lines")
                .default_value("10")
                .help("print number of lines")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("bytes")
                .short('b')
                .long("bytes")
                .default_value("0")
                .help("output number of bytes")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("number")
            .short('n')
            .long("number-of-lines")
            .help("output line numbers")
            .takes_value(false)
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("input file")
                .multiple(false)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .get_matches();

    let f = matches.value_of_lossy("files").unwrap();
    let l = matches.value_of_lossy("lines").unwrap().parse().unwrap();
    let b = matches.value_of_lossy("bytes").unwrap().parse().unwrap();
    let ln = matches.is_present("number");

    Arguments {
        filename: f.to_string(),
        lines: l,
        bytes: b,
        line_number: ln,
    }
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    let f = std::fs::File::open(args.filename)?;
    let mut r = std::io::BufReader::new(f);
    if args.bytes == 0 {
        for (index, line) in r.lines().enumerate() {
            match line {
                Ok(l) => {
                    if args.line_number == true {
                        println!("{}\t{}",index+1,l);
                    } else {
                        println!("{}", l);
                    }
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
            if index >= args.lines {
                break;
            }
        }
    } else {
        let mut s: String = String::new();
        r.read_to_string(&mut s).expect("on read");
        for (index, ch) in s.chars().enumerate() {
            print!("{}", ch);
            if index >= args.bytes {
                break;
            }
        }
        println!("");
    }
    Ok(())
}
