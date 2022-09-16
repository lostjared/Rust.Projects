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
                .required(false)
                .allow_invalid_utf8(true)
                .default_value("<STDIN>")
        )
        .get_matches();
    let f = m.value_of_lossy("file").unwrap();
    Arguments {
        in_file: f.to_string(),
    }
}

fn parse_text<T>(mut reader: T) 
where
    T: std::io::BufRead + Sized,
{

    let mut input: String = String::new();
    reader.read_to_string(&mut input).expect("read string");
    let mut rlex = rs_lex::rlex::Scanner::new(&input);

    while rlex.valid() {
        let token = rlex.scan_token();
        println!("{}", token.getString());
    }

}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    if args.in_file == "<STDIN>" {
        parse_text(std::io::stdin().lock());
    }
    Ok(())
}
