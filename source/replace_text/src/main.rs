// use format
// to find each instance of search and replace it with replace in standard input stream
// --input regex/serach
// --with  replace_with
// -r enable regex

use clap::{App, Arg};
use regex::Regex;
use std::io::BufRead;

struct Arguments {
    text: String,
    replace: String,
    re: bool,
    all: bool,
}

fn parse_args() -> Arguments {
    let m = App::new("replace_text")
        .author("Jared Bruni")
        .help("replace text")
        .version("0.1.0")
        .arg(
            Arg::with_name("text")
                .help("text use /search/replace format")
                .required(true)
                .multiple(false)
                .short('i')
                .long("input")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("replace")
            .help("replace with")
            .required(true)
            .multiple(false)
            .short('w')
            .long("with")
            .takes_value(true)
            .allow_invalid_utf8(true)
        )
        .arg(
            Arg::with_name("regex")
                .help("Use regex")
                .required(false)
                .takes_value(false)
                .multiple(false)
                .long("regex")
                .short('r'),
        )
        .arg( 
            Arg::with_name("all")
            .help("replace all")
            .required(false)
            .takes_value(false)
            .long("all")
            .short('a')
        )
        .get_matches();
    let t = m.value_of_lossy("text").unwrap();
    let rep = m.value_of_lossy("replace").unwrap();
    let r = m.is_present("regex");
    let a = m.is_present("all");
    Arguments {
        text: t.to_string(),
        replace: rep.to_string(),
        re: r,
        all: a,
    }
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    for i in std::io::stdin().lock().lines() {
        match i {
            Ok(line) => {
                if args.re {
                    let re = Regex::new(&args.text).unwrap();
                    if re.is_match(&line) {
                        if args.all {
                            let r = re.replace_all(&line, &args.replace);
                            println!("{}", r);
                        } else {
                            let r = re.replace(&line, &args.replace);
                            println!("{}", r);
                        }
                    } else {
                        println!("{}", line);
                    }
                } else {
                    let r = line.replace(&args.text, &args.replace);
                    println!("{}", r);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}
