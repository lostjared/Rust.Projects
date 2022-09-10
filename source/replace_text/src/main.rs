// use format
// /serach/replace
// to find each instance of serach and replace it with replace in standard input stream
// example:
// cat src/main.rs | cargo run /main/value

use clap::{App, Arg};
use std::io::BufRead;

struct Arguments {
    text: String,
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
        .get_matches();
    let t = m.value_of_lossy("text").unwrap();
    Arguments {
        text: t.to_string(),
    }
}

fn replace_text(input: &str, search: &str) -> Option<String> {
    let pos1 = search.find("/");
    if pos1 == None {
        return None;
    }
    let left_of = &search[pos1.unwrap()+1..];
    let pos2 = left_of.find("/");
    if pos2 == None {
        return None;
    }
    let search_val = &left_of[..pos2.unwrap()];
    let rtext = &left_of[pos2.unwrap()+1..];
    let input_text = String::from(input);
    let success = input_text.replace(search_val, rtext);
    Some(success)
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    for i in std::io::stdin().lock().lines() {
        match i {
            Ok(line) => {
                let r = replace_text(&line, &args.text);
                if r == None {
                    println!("{}", line);
                } else {
                    println!("{}", r.unwrap());
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}
