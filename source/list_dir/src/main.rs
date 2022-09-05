use clap::{App, Arg};

struct Arguments {
    paths: Vec<String>,
}

fn parse_args() -> Arguments {
    let m = App::new("list_dir")
        .author("Jared")
        .help("List paths")
        .version("0.1.0")
        .arg(
            Arg::with_name("paths")
                .value_name("FILE_PATH")
                .help("input path(s)")
                .multiple(true)
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .get_matches();
    let p: Vec<String> = m.values_of_lossy("paths").unwrap();
    Arguments { paths: p }
}

fn main() -> std::io::Result<()> {
    Ok(())
}
