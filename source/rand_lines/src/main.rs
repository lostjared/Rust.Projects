use clap::{App, Arg};
use rand::Rng;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;

struct Arguments {
    input: String,
    output: String,
    num: usize,
    len: usize,
}

fn parse_args() -> Arguments {
    let m = App::new("rand_lines")
        .help("random lines")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("input file")
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("output file")
                .takes_value(true)
                .required(false)
                .default_value("<STDOUT>")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .help("number of lines")
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("len")
                .short('l')
                .long("len")
                .help("length of lines")
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .get_matches();

    let input_ = m.value_of_lossy("input").unwrap();
    let output_ = m.value_of_lossy("output").unwrap();
    let num_ = m.value_of_lossy("num").unwrap().parse().unwrap();
    let len_ = m.value_of_lossy("len").unwrap().parse().unwrap();

    Arguments {
        input: input_.to_string(),
        output: output_.to_string(),
        num: num_,
        len: len_,
    }
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    let mut v: Vec<String> = Vec::new();
    let r = std::io::BufReader::new(std::fs::File::open(args.input).unwrap());
    let mut map: HashMap<String, String> = HashMap::new();
    for line in r.lines() {
        match line {
            Ok(l) => {
                if map.contains_key(&l) {
                    continue;
                }
                if l.len() > args.len {
                    v.push(l.to_owned());
                    map.insert(l.to_owned(), l);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    let mut rng = rand::thread_rng();
    let mut count = 0;
    let mut w: std::io::BufWriter<Box<dyn std::io::Write>>;
    if args.output != "<STDOUT>" {
        let f = std::fs::File::create(args.output).unwrap();
        w = std::io::BufWriter::new(Box::new(f));
    } else {
        w = std::io::BufWriter::new(Box::new(std::io::stdout().lock()));
    }
    for _i in 0..args.num {
        let r = rng.gen_range(0..v.len());
        let line = &v[r];
        writeln!(w, "{}", line).expect("on write");
        count += 1;
        if count > args.num - 1 {
            break;
        }
        v.remove(r);
    }
    w.flush().expect("on flush");
    println!("Generated {} Line(s)\n", count - 1);
    Ok(())
}
