use clap::{App, Arg};

struct Arguments {
    filename: String,
    index: usize,
    depth: usize,
}

fn parse_args() -> Arguments {
    let matches = App::new("filter")
        .about("Filter image example")
        .author("Jared Bruni")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("input file")
                .multiple(false)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("index")
                .help("filter index")
                .short('c')
                .long("index")
                .required(true),
        )
        .arg(
            Arg::with_name("depth")
                .help("Filter Depth")
                .short('c')
                .long("Depth")
                .required(true),
        )
        .get_matches();

    let f = matches.value_of("file").unwrap();
    let ind = matches.value_of("index").unwrap().parse().unwrap();
    let dept = matches.value_of("depth").unwrap().parse().unwrap();

    Arguments {
        filename: f.to_string(),
        index: ind,
        depth: dept,
    }
}

fn main() -> std::io::Result<()> {
    Ok(())
}
