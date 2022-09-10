// use format
// /search/replace
// to find each instance of search and replace it with replace in standard input stream
// example:
// cat src/main.rs | cargo run "/main/value"
// or use
// cat src/main.rs | cargo run "#main#value" --sep #
// to change the character to seperate by

use clap::{App, Arg};
use std::io::BufRead;

struct Arguments {
    text: String,
    sep: char,
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
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("sep")
                .help("seperation character")
                .required(false)
                .takes_value(true)
                .multiple(false)
                .long("sep")
                .short('s')
                .default_value("/")
                .allow_invalid_utf8(true),
        )
        .get_matches();
    let t = m.value_of_lossy("text").unwrap();
    let sep = m.value_of_lossy("sep").unwrap().to_string();
    let ch: char = sep.chars().nth(0).unwrap();
    Arguments {
        text: t.to_string(),
        sep: ch,
    }
}

fn extract_search(search: &str, sep: char) -> Option<(String, String)> {
    let pos1 = search.find(sep);
    if pos1 == None {
        return None;
    }
    let left_of = &search[pos1.unwrap() + 1..];
    let pos2 = left_of.find(sep);
    if pos2 == None {
        return None;
    }
    let search_val = &left_of[..pos2.unwrap()];
    let rtext = &left_of[pos2.unwrap() + 1..];
    Some((search_val.to_string(), rtext.to_string()))
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    let search_values = extract_search(&args.text, args.sep);
    if search_values == None {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid text string",
        ));
    }
    let search_values = search_values.unwrap();
    for i in std::io::stdin().lock().lines() {
        match i {
            Ok(line) => {
                let r = line.replace(&search_values.0, &search_values.1);
                println!("{}", r);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}
