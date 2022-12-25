use clap::{App, Arg};

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

fn convert_to_rs<T: std::io::BufRead + Sized>(mut reader: T) -> String {
    let mut value: String = String::new();
    value.push_str("let v = vec![");
    loop {
        let mut input_text: String = String::new();
        let val = reader.read_line(&mut input_text).expect("on read");
        input_text.pop();
        value.push_str(&format!("\n\"{}\"", &slash_seq(&input_text)));
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
    value.push_str(&format!("std::vector<std::string> {} = {{", name));
    loop {
        let mut input_text: String = String::new();
        let val = reader.read_line(&mut input_text).expect("on read");
        input_text.pop();
        value.push_str(&format!("\n\"{}\"", &slash_seq(&input_text)));
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
        .get_matches();
    let c = m.is_present("cxx");
    let filen = m.values_of_lossy("file").unwrap();
    Arguments {
        cxx: c,
        filename: filen,
    }
}

fn main() {
    let arg_m = parse_args();
    let f1 = arg_m.filename.get(0).unwrap();
    if f1 == "<STDIN>" {
        let i = std::io::stdin();
        let r = i.lock();

        let s: String = if !arg_m.cxx {
            convert_to_rs(r)
        } else {
            convert_to_cxx(r, "v")
        };
        println!("{}", s);
    } else {

        let mut index = 0;

        for i in arg_m.filename {
            index += 1;
            let f = std::fs::File::open(i).unwrap();
            let r = std::io::BufReader::new(f);
            let s: String = if !arg_m.cxx {
                convert_to_rs(r)
            } else {
                convert_to_cxx(r, &format!("v{}", index))
            };
            println!("{}", s);
        }
    }
}
