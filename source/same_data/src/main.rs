use clap::{App, Arg};
use std::collections::HashMap;

struct Arguments {
    files: Vec<String>,
}

fn parse_args() -> Arguments {
    let m = App::new("same_data")
        .author("Jared Bruni")
        .help("Same data")
        .version("0.1.0")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("input file(s)")
                .multiple(true)
                .default_value("<STDIN>")
                .allow_invalid_utf8(true),
        )
        .get_matches();
    let v = m.values_of_lossy("files").unwrap();
    Arguments { files: v }
}

fn fill_map<T: std::io::BufRead + Sized>(r: T, m: &mut HashMap<String, i32>) {
    for i in r.lines() {
        match i {
            Ok(line) => {
                if line.trim().len() == 0 {
                    continue;
                }

                if m.contains_key(&line) {
                    let val = m.get(&line).unwrap();
                    m.insert(line, val + 1);
                } else {
                    m.insert(line, 1);
                }
            }
            Err(e) => {
                println!("Errror: {}", e);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    let mut map: HashMap<String, i32> = HashMap::new();
    for i in &args.files {
        if i == "<STDIN>" {
            fill_map(std::io::stdin().lock(), &mut map);
        } else {
            let f = std::fs::File::open(i)?;
            let r = std::io::BufReader::new(f);
            fill_map(r, &mut map);
        }
    }
    for (key, value) in map {
        println!("{}:\t{}", key, value);
    }
    Ok(())
}
