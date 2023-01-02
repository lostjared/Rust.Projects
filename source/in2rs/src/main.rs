// practice project:
// in2rs - command line tool convert text to Rust/C++

use clap::{App, Arg};
use colored::Colorize;
use regex::Regex;
use std::io::Write;

/// slash sequence convert slashes
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

/// convert steam to Rust
fn convert_to_rs<T: std::io::BufRead + Sized>(mut reader: T, name: &str, blank: bool) -> String {
    use std::fmt::Write;
    let mut value: String = String::new();
    write!(&mut value, "let {} = vec![", name).expect("on write");
    loop {
        let mut input_text: String = String::new();
        let val = reader.read_line(&mut input_text).expect("on read");
        if input_text == "\n" && blank == true {
            continue;
        }
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

/// convert stream tO C++
fn convert_to_cxx<T: std::io::BufRead + Sized>(mut reader: T, name: &str, blank: bool) -> String {
    use std::fmt::Write;
    let mut value: String = String::new();
    write!(&mut value, "std::vector<std::string> {} = {{", name).expect("on write");
    loop {
        let mut input_text: String = String::new();
        let val = reader.read_line(&mut input_text).expect("on read");
        if input_text == "\n" && blank == true {
            continue;
        }
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
    blank: bool,
}

/// parse arguments return struct
fn parse_args() -> Arguments {
    let m = App::new("in2rs")
        .help("in2rs")
        .author("Jared Bruni")
        .version("0.1")
        .arg(
            Arg::new("cxx")
                .short('c')
                .long("cxx")
                .takes_value(false)
                .required(false)
                .help("output as C++"),
        )
        .arg(
            Arg::new("file")
                .takes_value(true)
                .multiple(true)
                .required(false)
                .default_value("<STDIN>")
                .allow_invalid_utf8(true)
                .help("input file(s)"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .multiple(false)
                .required(false)
                .help("Output to C++ file")
                .default_value("<NONE>")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("no-blank")
                .short('n')
                .long("no-blank")
                .takes_value(false)
                .multiple(false)
                .required(false)
                .help("no blanks"),
        )
        .get_matches();
    let c = m.is_present("cxx");
    let filen = m.values_of_lossy("file").unwrap();
    let out = m.value_of_lossy("output").unwrap();
    let n = m.is_present("no-blank");

    Arguments {
        cxx: c,
        filename: filen,
        output: out.to_string(),
        blank: n,
    }
}

/// output code header for C++
fn output_code_header(name: &str, filen: &Vec<String>) {
    let name_hxx = format!("{}.hpp", name);
    let f = std::fs::File::create(name_hxx).expect("on create");
    let mut w = std::io::BufWriter::new(f);
    writeln!(&mut w, "#ifndef SOURCE_CXX_{}_H", name).expect("on write");
    writeln!(&mut w, "#define SOURCE_CXX_{}_H", name).expect("on write");
    writeln!(&mut w, "#include<string>\n#include<vector>\n").expect("on write");
    for index in 0..filen.len() {
        writeln!(&mut w, "extern std::vector<std::string> v{};\n", index + 1).expect("on write");
    }
    writeln!(&mut w, "#endif").expect("on write");
}

/// output code to stream Rust/C++
fn output_code_to_stream<T: std::io::Write + Sized>(mut writer: T, files: &Vec<String>, cxx: bool, blank: bool) {
    let mut index = 0;
    for i in files {
        index += 1;
        let f = std::fs::File::open(i).unwrap();
        let r = std::io::BufReader::new(f);
        let s: String = if !cxx {
            convert_to_rs(r, &format!("v{}", index), blank)
        } else {
            convert_to_cxx(r, &format!("v{}", index), blank)
        };
        writeln!(&mut writer, "{}", s).expect("on write");
    }
}

/// output Rust code to stream
fn output_rs_code_to_stream<T: std::io::Write + Sized>(mut writer: T, files: &Vec<String>, blank: bool) {
    let mut index = 0;
    for i in files {
        index += 1;
        let f = std::fs::File::open(i).unwrap();
        let r = std::io::BufReader::new(f);
        let s = convert_to_rs(r, &format!("v{}", index), blank);
        writeln!(
            &mut writer,
            "fn init_v{}() -> Vec<&\'static str>\n{{\n{}\nv{}\n}}\n",
            index, s, index
        )
        .expect("on write");
    }
}

/// main function
fn main() {
    let arg_m = parse_args();
    let f1 = arg_m.filename.get(0).unwrap();
    if f1 == "<STDIN>" {
        let i = std::io::stdin();
        let r = i.lock();
        let s: String = if !arg_m.cxx {
            convert_to_rs(r, "v", arg_m.blank)
        } else {
            convert_to_cxx(r, "v", arg_m.blank)
        };
        println!("{}", s);
    } else {
        if arg_m.output != "<NONE>" {
            let re = Regex::new(r"[A-Za-z][A-Za-z0-9]*").unwrap();
            if !re.is_match(&arg_m.output) {
                panic!("Error invalid output variable name");
            }
        }
        if arg_m.output != "<NONE>" && arg_m.cxx {
            let name_cxx = format!("{}.cpp", arg_m.output);
            let name_hxx = format!("{}.hpp", arg_m.output);
            output_code_header(&arg_m.output, &arg_m.filename);
            println!("header file: {}", name_hxx);
            let f = std::fs::File::create(name_cxx).expect("on create");
            let mut w = std::io::BufWriter::new(f);
            writeln!(&mut w, "#include \"{}\"\n", name_hxx).expect("on write");
            output_code_to_stream(w, &arg_m.filename, arg_m.cxx, arg_m.blank);
            println!("source file: {}.cpp", arg_m.output);
        } else if arg_m.output != "<NONE>" && !arg_m.cxx {
            let name_rs = format!("{}.rs", arg_m.output);
            let f = std::fs::File::create(name_rs).expect("on create");
            let mut w = std::io::BufWriter::new(f);
            output_rs_code_to_stream(&mut w, &arg_m.filename, arg_m.blank);
            println!("source file: {}.rs", arg_m.output);
        } else {
            println!("{}:", "Output".red());
            output_code_to_stream(std::io::stdout().lock(), &arg_m.filename, arg_m.cxx, arg_m.blank);
        }
    }
}
