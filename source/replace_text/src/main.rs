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
                .help("text")
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

fn main() -> std::io::Result<()> {
    let args = parse_args();

    for i in std::io::stdin().lock().lines() {
        match i {
            Ok(line) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}
