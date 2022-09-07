use clap::{App, Arg};
use std::collections::HashMap;

struct Arguments {
    files: Vec<String>,
}

fn parse_args() -> Arguments {
    let matches = App::new("common-lines")
        .help("print out common lines")
        .author("Jared")
        .version("0.1.0")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("input file(s)")
                .multiple(true)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .get_matches();
    let v: Vec<String> = matches.values_of_lossy("files").unwrap();
    Arguments { files: v }
}

fn fill_map(input: &str, map: &mut HashMap<String, u32>) {



}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    if args.files.len() < 2 {
        eprintln!("Error requires at least two files...\n");
        return Ok(());
    }
    let mut map : HashMap<String, u32> = HashMap::new();
    for i in &args.files {
        fill_map(i, &mut map);
    }

    Ok(())
}
