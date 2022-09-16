use clap::{App, Arg};

struct Arguments {
    in_file: String,
}

fn parse_args() -> Arguments {
    let m = App::new("rs_lex")
        .help("rs_lex")
        .about("rs_lex")
        .author("Jared Bruni <jaredbruni@protonmail.com>")
        .arg(
            Arg::with_name("file")
                .multiple(false)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .get_matches();
    let f = m.value_of_lossy("file").unwrap();
    Arguments {
        in_file: f.to_string(),
    }
}

fn main() -> std::io::Result<()> {
    let args = parse_args();

    Ok(())
}
