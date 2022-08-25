use clap::{App, Arg};
use std::io::BufRead;

#[derive(Debug)]
struct Arguments {
    filename: String,
    lines: usize,
    bytes: usize,
}

fn parse_args() -> Arguments {
    let matches = App::new("start_text")
        .version("0.1.0")
        .author("Jared Bruni")
        .help("Print starting text")
        .arg(
            Arg::with_name("lines")
                .short('l')
                .default_value("10")
                .help("print number of lines")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("bytes")
                .short('b')
                .default_value("0")
                .help("output number of bytes")
                .allow_invalid_utf8(true),
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

    Arguments {
        filename: f.to_string(),
        lines: l,
        bytes: b,
    }
}

fn main() -> std::io::Result<()> {
    let args = parse_args();

    let f = std::fs::File::open(args.filename)?;
    let r = std::io::BufReader::new(f);

    if args.bytes == 0 {
        for (index, line) in r.lines().enumerate() {
            match line {
                Ok(l) => {
                    println!("{}", l);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
            if index >= args.lines {
                break;
            }
        }
    }

    Ok(())
}
