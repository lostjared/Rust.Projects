use wasm_bindgen::prelude::*;
use std::io::BufReader;

#[wasm_bindgen]
pub fn convert_text(text: &str) -> String {
    let reader = BufReader::new(text.as_bytes());
    convert_to_cxx(reader,"v",true)
}

#[wasm_bindgen]
pub fn convert_text_v(text: &str) -> String {
    let reader = BufReader::new(text.as_bytes());
    convert_to_cxx_v(reader)
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

fn convert_to_cxx_v<T>(mut reader: T) -> String
where
    T: std::io::BufRead + Sized,
{
    let mut value: String = String::new();
    value.push_str("std::vector<char> v {");
    let mut bytes = [0; 1024];
    let mut first_byte = true;

    loop {
        let val = reader.read(&mut bytes).expect("Failed to read");
        if val == 0 {
            break;
        }
        if !first_byte {
            value.push(',');
        }
        first_byte = false;

        for i in 0..val {
            if i > 0 {
                value.push(',');
            }
            let s = format!("{:#04x}", bytes[i]);
            value.push_str(&s);
        }
    }
    value.push_str("};\n");
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

