// use format
// /search/replace
// to find each instance of search and replace it with replace in standard input stream
// example:
// cat src/main.rs | cargo run "/main/value"

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

fn extract_search(search: &str) -> Option<(String, String)> {
    let pos1 = search.find('/');
    if pos1 == None {
        return None;
    }
    let left_of = &search[pos1.unwrap() + 1..];
    let pos2 = left_of.find('/');
    if pos2 == None {
        return None;
    }
    let search_val = &left_of[..pos2.unwrap()];
    let rtext = &left_of[pos2.unwrap() + 1..];
    Some((search_val.to_string(), rtext.to_string()))
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    let search_values = extract_search(&args.text);
    if search_values == None {
        eprintln!("Error invalid search string");
        return Ok(());
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
