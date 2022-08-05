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

fn convert_to_rs() -> String {
    let mut value: String = String::new();
    value.push_str("let v = vec![");
    loop {
        let mut input_text: String = String::new();
        let val = std::io::stdin()
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
    let s: String = convert_to_rs();
    println!("{}", s);
}
