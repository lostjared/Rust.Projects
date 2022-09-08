use clap::{App, Arg};

struct Arguments {
    file: String,
    bytes: u32,
}

fn parse_args() -> Arguments {
    let m = App::new("end_of_file")
        .author("jared")
        .help("end of file")
        .version("0.1.0")
        .arg(
            Arg::with_name("bytes")
                .required(true)
                .multiple(false)
                .long("bytes")
                .short('b')
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("file")
                .required(true)
                .multiple(false)
                .allow_invalid_utf8(true),
        )
        .get_matches();

    let f = m.value_of_lossy("file").unwrap();
    let b = m.value_of_lossy("bytes").unwrap().parse().unwrap();

    Arguments {
        file: f.to_string(),
        bytes: b,
    }
}

fn main() -> std::io::Result<()> {

    let args = parse_args();

    Ok(())
}
