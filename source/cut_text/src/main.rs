use clap::{App, Arg};

struct Arguments {
    file: String,
    start: usize,
    number: usize,
    mode: usize,
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
                .multiple(false)
                .default_value("<STDIN>")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("cut")
                .value_name("CUT")
                .help("cut start,number")
                .multiple(false)
                .allow_invalid_utf8(true)
                .required(false)
                .short('c')
                .long("cut")
                .conflicts_with("substr"),
        )
        .arg(
            Arg::with_name("substr")
                .value_name("SUBSTR")
                .help("substring")
                .multiple(false)
                .allow_invalid_utf8(true)
                .required(false)
                .short('s')
                .long("substr")
                .conflicts_with("cut"),
        )
        .get_matches();

    let f = m.value_of_lossy("files").unwrap();
    let s1 = m.value_of_lossy("cut");
    let s2 = m.value_of_lossy("substr");
    let s;
    let mode_val;
    if s1 != None {
        s = s1.unwrap();
        mode_val = 1;
    } else {
        s = s2.unwrap();
        mode_val = 2;
    }

    let pos = s.find(',').unwrap();
    let left = &s[..pos];
    let st: usize = left.parse().unwrap();
    let right = &s[pos + 1..];
    let n: usize = right.parse().unwrap();
    Arguments {
        file: f.to_string(),
        start: st,
        number: n,
        mode: mode_val,
    }
}

fn cut_value<T>(mut reader: T, start: usize, num: usize, mode: usize) -> String
where
    T: std::io::BufRead + Sized,
{
    let mut s = String::new();
    let _v = reader.read_to_string(&mut s).expect("on read");
    cut_text(&s, start, num, mode)
}

fn cut_text(input: &str, start: usize, num: usize, mode: usize) -> String {
    let d = if mode == 1 {
        &input[start..start + num]
    } else {
        &input[start..num]
    };
    String::from(d)
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    if args.file == "<STDIN>" {
        println!(
            "{}",
            cut_value(std::io::stdin().lock(), args.start, args.number, args.mode)
        );
    } else {
        let f = std::fs::File::open(args.file)?;
        let r = std::io::BufReader::new(f);
        println!("{}", cut_value(r, args.start, args.number, args.mode));
    }
    Ok(())
}
