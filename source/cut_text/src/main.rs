use clap::{App, Arg};

struct Arguments {
    file: String,
    start: usize,
    number: usize,
}

fn parse_args() -> Arguments {
    let m = App::new("cut_text")
        .author("Jared")
        .help("cut text")
        .version("0.1.0")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("input file(s)")
                .multiple(true)
                .default_value("<STDIN>")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("cut")
                .value_name("CUT")
                .help("cut start,number")
                .multiple(false)
                .allow_invalid_utf8(true)
                .required(true)
                .short('c')
                .long("cut"),
        )
        .get_matches();

    let f = m.value_of_lossy("files").unwrap();
    let s = m.value_of_lossy("cut").unwrap();
    let pos = s.find(',').unwrap();
    let left = &s[..pos];
    let st: usize = left.parse().unwrap();
    let right = &s[pos+1..];
    let n: usize = right.parse().unwrap();
    Arguments {
        file: f.to_string(),
        start: st,
        number: n,
    }
}

fn cut_value<T>(mut reader: T, start: usize, num: usize) -> String
where
    T: std::io::BufRead + Sized,
{

    let mut s = String::new();
    let _v = reader.read_to_string(&mut s).expect("on read");
    cut_text(&s, start, num)

}

fn cut_text(input: &str, start: usize, num: usize) -> String {
    let d = &input[start..start + num];
    String::from(d)
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    if args.file == "<STDIN>" {
        println!("{}", cut_value(std::io::stdin().lock(), args.start, args.number));
    } else {
        let f = std::fs::File::open(args.file)?;
        let r = std::io::BufReader::new(f);
        println!("{}", cut_value(r, args.start, args.number));
    }
    Ok(())
}
