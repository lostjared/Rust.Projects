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

fn count_lines<T>(r: T) -> usize
where
    T: std::io::BufRead + Sized,
{
    let count = r.lines().count();
    count
}

fn count_chars<T>(mut r: T) -> usize
where
    T: std::io::BufRead + Sized,
{
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
            _ => {}
        }
    }
    output
}

fn count_words<T>(mut r: T) -> usize
where
    T: std::io::BufRead + Sized,
{
    let mut input = String::new();
    let _len = r.read_to_string(&mut input).expect("on read");
    let val = remove_chars(&input);
    let values = val.split(" ");
    values.count()
}

fn count_text(args: &Arguments) -> std::io::Result<()> {
    for i in &args.filenames {
        if args.lines {
            let c;
            if i == "." {
                let r = std::io::stdin().lock();
                c = count_lines(r);
            } else {
                let f = std::fs::File::open(i)?;
                c = count_lines(std::io::BufReader::new(f));
            }
            println!("\tlines: {}", c);
        }
        if args.ch {
            let c;
            if i == "." {
                let r = std::io::stdin().lock();
                c = count_chars(r);
            } else {
                let f = std::fs::File::open(i)?;
                c = count_chars(std::io::BufReader::new(f));
            }
            println!("\tbytes: {}", c);
        }
        if args.words {
            let c;
            if i == "." {
                let r = std::io::stdin().lock();
                c = count_words(r);
            } else {
                let f = std::fs::File::open(i)?;
                c = count_words(std::io::BufReader::new(f));
            }
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
