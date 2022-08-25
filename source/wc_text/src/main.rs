use clap::{App, Arg};

struct Arguments {
    filenames: Vec<String>,
    lines: bool,
    ch: bool,
    words: bool,
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
                .help("print number of lines"),
        )
        .arg(
            Arg::with_name("bytes")
                .short('b')
                .long("bytes")
                .help("output number of bytes"),
        )
        .arg(
            Arg::with_name("words")
                .short('w')
                .long("words")
                .help("output number of words"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("input file")
                .default_value(".")
                .multiple(true)
                .required(false)
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

fn remove_chars(input: &str) -> String {
    let mut output = String::new();
    for i in input.chars() {
        match i {
            'a'..='z' | 'A'..='Z' | ' ' | '\n' => {
                output.push(i);
            }
            _ => {}
        }
    }
    output
}

fn count_data<T>(mut r: T) -> (usize, usize, usize)
where
    T: std::io::BufRead + Sized,
{
    let mut input = String::new();
    let _len = r.read_to_string(&mut input).expect("on read");
    let val = remove_chars(&input);
    let values = val.split(" ");
    (input.lines().count(), input.len(), values.count())
}

fn count_text(args: &Arguments) -> std::io::Result<()> {
    for i in &args.filenames {
        let val;
        if i == "." {
            val = count_data(std::io::stdin().lock());
        } else {
            let f = std::fs::File::open(i)?;
            let r = std::io::BufReader::new(f);
            val = count_data(r);
        }
        if args.lines {
            println!("\tlines: {}", val.0);
        }
        if args.ch {
            println!("\tbytes: {}", val.1);
        }
        if args.words {
            println!("\twords; {}", val.2);
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    count_text(&args)?;
    Ok(())
}
