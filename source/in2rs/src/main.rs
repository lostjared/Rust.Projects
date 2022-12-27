use clap::{App, Arg};
use colored::Colorize;
use std::fmt::Write;

fn slash_seq(input: &str) -> String {
    let mut value: String = String::new();
    for i in input.chars() {
        if i == '\\' {
            value.push_str("\\\\");
        } else if i == '\"' {
            value.push_str("\\\"");
        } else {
            value.push(i);
        }
    }
    value
}

fn convert_to_rs<T: std::io::BufRead + Sized>(mut reader: T, name: &str) -> String {
    let mut value: String = String::new();
    write!(&mut value, "{} {} = vec![", "let".blue(), name).expect("on write");
    loop {
        let mut input_text: String = String::new();
        let val = reader.read_line(&mut input_text).expect("on read");
        input_text.pop();
        write!(&mut value, "\n\"{}\"", &slash_seq(&input_text)).expect("on write");
        if val == 0 {
            break;
        } else {
            value.push(',');
        }
    }
    value.push_str("];\n");
    value
}

fn convert_to_cxx<T: std::io::BufRead + Sized>(mut reader: T, name: &str) -> String {
    let mut value: String = String::new();
    write!(&mut value, "std::vector<std::string> {} = {{", name).expect("on write");
    loop {
        let mut input_text: String = String::new();
        let val = reader.read_line(&mut input_text).expect("on read");
        input_text.pop();
        write!(&mut value, "\n\"{}\"", &slash_seq(&input_text)).expect("on write");
        if val == 0 {
            break;
        } else {
            value.push(',');
        }
    }
    value.push_str("};\n");
    value
}

struct Arguments {
    cxx: bool,
    filename: Vec<String>,
    output: String,
}

fn parse_args() -> Arguments {
    let m = App::new("in2rs")
        .help("in2rs")
        .arg(
            Arg::new("cxx")
                .short('c')
                .long("cxx")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::new("file")
                .takes_value(true)
                .multiple(true)
                .required(false)
                .default_value("<STDIN>")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .multiple(false)
                .required(false)
                .default_value("<NONE>")
                .allow_invalid_utf8(true),
        )
        .get_matches();
    let c = m.is_present("cxx");
    let filen = m.values_of_lossy("file").unwrap();
    let out = m.value_of_lossy("output").unwrap();

    Arguments {
        cxx: c,
        filename: filen,
        output: out.to_string(),
    }
}

fn main() {
    let arg_m = parse_args();
    let f1 = arg_m.filename.get(0).unwrap();
    if f1 == "<STDIN>" {
        let i = std::io::stdin();
        let r = i.lock();

        let s: String = if !arg_m.cxx {
            convert_to_rs(r, "v")
        } else {
            convert_to_cxx(r, "v")
        };
        println!("{}:\n{}", "Output".red(), s);
    } else {
        let mut index = 0;
        for i in arg_m.filename {
            index += 1;
            let f = std::fs::File::open(i).unwrap();
            let r = std::io::BufReader::new(f);
            let s: String = if !arg_m.cxx {
                convert_to_rs(r, &format!("v{}", index))
            } else {
                convert_to_cxx(r, &format!("v{}", index))
            };
            println!("{}:\n{}", "Output".red(), s);
        }
    }
}
