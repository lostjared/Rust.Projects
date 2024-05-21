use wasm_bindgen::prelude::*;
use std::io::BufReader;

#[wasm_bindgen]
pub fn convert_text(text: &str) -> String {
    let reader = BufReader::new(text.as_bytes());
    convert_to_rs(reader)
}

fn convert_to_rs<T>(mut reader: T) -> String
where
    T: std::io::BufRead + Sized,
{
    let mut value: String = String::new();
    value.push_str("let v : Vec<u8> = vec![");
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
    value.push_str("];\n");
    value
}