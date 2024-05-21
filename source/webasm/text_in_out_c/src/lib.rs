    use wasm_bindgen::prelude::*;
use std::io::BufReader;

#[wasm_bindgen]
pub fn convert_text(text: &str) -> String {
    let reader = BufReader::new(text.as_bytes());
    convert_to_cxx(reader,"v",true)
}

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

fn convert_to_cxx<T: std::io::BufRead + Sized>(mut reader: T, name: &str, blank: bool) -> String {
    use std::fmt::Write;
    let mut value: String = String::new();
    write!(&mut value, "std::vector<std::string> {} {{", name).expect("on write");
    loop {
        let mut input_text: String = String::new();
        let val = reader.read_line(&mut input_text).expect("on read");
        if input_text == "\n" && blank == true {
            continue;
        }
        input_text.pop();
        if input_text.len() > 0 || blank == false {
            write!(&mut value, "\n\"{}\"", &slash_seq(&input_text)).expect("on write");
            if val == 0 {
                break;
            } else {
                value.push(',');
            }
        } else if val == 0 {
            value.remove(value.len() - 1);
            break;
        }
    }
    value.push_str("\n};\n");
    value
}