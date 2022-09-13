use clap::{App, Arg};

struct Arguments {
    files: Vec<String>,
}

fn parse_args() -> Arguments {
    let m = App::new("sort_")
        .author("Jared")
        .help("sort data")
        .version("0.1.0")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .default_value("<STDIN>")
                .multiple(true)
                .allow_invalid_utf8(true),
        )
        .get_matches();
    let v = m.values_of_lossy("files").unwrap();
    Arguments { files: v }
}

fn read_stream<T>(reader: T, v: &mut Vec<String>)
where
    T: std::io::BufRead + Sized,
{
    for i in reader.lines() {
        let i = i.unwrap();
        if !i.is_empty() {
            v.push(i);
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    let mut v: Vec<String> = Vec::new();
    for i in &args.files {
        if i == "<STDIN>" {
            read_stream(std::io::stdin().lock(), &mut v);
        } else {
            let f = std::fs::File::open(i)?;
            let r = std::io::BufReader::new(f);
            read_stream(r, &mut v);
        }
    }
    v.sort();
    v.into_iter().for_each(|i| println!("{}", i));
    Ok(())
}
