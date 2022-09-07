use clap::{App, Arg};
use std::collections::BTreeMap;
use std::io::BufRead;

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

fn fill_map(input: &str, map: &mut BTreeMap<String, u32>) {
    let mut cur_map : BTreeMap<String, u32> = BTreeMap::new();
    let f = std::fs::File::open(input).expect("on open file");
    let r = std::io::BufReader::new(f);
    for i in r.lines() {
        match i {
            Ok(line) => {
                if !cur_map.contains_key(&line) {
                    cur_map.insert(line, 0);
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    for (key,_value) in &cur_map {
        if map.contains_key(key) {
            let v = map[key];
            map.insert(key.to_string(), v+1); 
        } else {
            map.insert(key.to_string(), 0);
        }
    }

}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    if args.files.len() < 2 {
        eprintln!("Error requires at least two files...\n");
        return Ok(());
    }
    let mut map : BTreeMap<String, u32> = BTreeMap::new();
    for i in &args.files {
        fill_map(i, &mut map);
    }
    for (key, value) in &map {
        if *value >= 1 {
            println!("{}", key);
        }
    }
    Ok(())
}
