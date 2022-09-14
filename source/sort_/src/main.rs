use clap::{App, Arg};

struct Arguments {
    files: Vec<String>,
    rev: bool,
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
        .arg(
            Arg::with_name("reverse")
            .multiple(false)
            .takes_value(false)
            .required(false)
            .short('r')
            .long("reverse")
        )
        .get_matches();
    let v = m.values_of_lossy("files").unwrap();
    let b = m.is_present("reverse");
    Arguments { files: v, rev: b }
}

fn read_stream<T>(reader: T, v: &mut Vec<String>)
where
    T: std::io::BufRead + Sized,
{
    reader.lines().for_each(|i| {
        let i = i.unwrap();
        if !i.is_empty() {
            v.push(i);
        }
    });
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    let mut v: Vec<String> = Vec::new();
    args.files.into_iter().for_each(|i| {
        if i == "<STDIN>" {
            read_stream(std::io::stdin().lock(), &mut v);
        } else {
            read_stream(std::io::BufReader::new(std::fs::File::open(i).unwrap()), &mut v);
        }
    });
    if args.rev {
        v.sort_by(|a, b| b.cmp(a));
    } else {
        v.sort();
    }
    v.into_iter().for_each(|i| println!("{}", i));
    Ok(())
}
