use std::io::BufRead;
use std::io::BufReader;

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

fn convert_to_rs<T: BufRead + Sized>(mut reader: T) -> String {
    let mut value: String = String::new();
    value.push_str("let v = vec![");
    loop {
        let mut input_text: String = String::new();
        let val = reader
            .read_line(&mut input_text)
            .expect("on read");
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

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        let i = std::io::stdin();
        let r = i.lock();
        let s: String = convert_to_rs(r);
        println!("{}", s);
    } else {
        for i in args.iter().skip(1) {
            let f = std::fs::File::open(i).unwrap();
            let r = BufReader::new(f);
            let s: String = convert_to_rs(r);
            println!("{}", s);
        }
    }
}
