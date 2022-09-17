use std::collections::HashMap;
use std::io::Read;
use std::io::Write;

fn convert_to_slash(input: &String) -> String {
    let mut s = String::new();
    let mut i = 0;
    while i < input.len() {
        let ch = input.chars().nth(i).unwrap();
        i += 1;
        match ch {
            '\\' => {
                let chx = input.chars().nth(i).unwrap();
                s.push(ch);
                s.push(chx);
                i += 1;
            }
            '\"' => {
                s.push_str("\\\"");
            }
            '\'' => {
                s.push_str("\\\'");
            }
            '\n' => {
                s.push_str("\\n");
            }
            '\r' => {
                s.push_str("\\r");
            }
            '\t' => {
                s.push_str("\\t");
            }
            _ => {
                s.push(ch);
            }
        }
    }
    s
}

fn convert_from_slash(input: String) -> String {
    let mut s: String = String::new();
    let mut i = 0;
    while i < input.len() {
        let ch = input.chars().nth(i).unwrap();
        i += 1;
        match ch {
            '\\' => {
                let chx = input.chars().nth(i).unwrap();
                match chx {
                    'n' => {
                        s.push('\n');
                    }
                    't' => {
                        s.push('\t');
                    }
                    'r' => {
                        s.push('\r');
                    }
                    '\"' => {
                        s.push('\"');
                    }
                    '\'' => {
                        s.push('\'');
                    }
                    _ => {
                        s.push(chx);
                    }
                }
                i += 1;
            }
            _ => {
                s.push(ch);
            }
        }
    }
    s
}

fn save_map(out_file: &str, map: &HashMap<String, String>) -> std::io::Result<()> {
    let f = std::fs::File::create(out_file)?;
    let mut w = std::io::BufWriter::new(f);
    let mut s: String = String::new();
    s.push_str("map = {\n");
    for (key, value) in map.iter() {
        s.push_str(&format!(
            "\"{}\" = \"{}\"\n",
            convert_to_slash(key),
            convert_to_slash(value)
        ));
    }
    s.push_str("}\n");
    w.write(s.as_bytes())?;
    Ok(())
}

fn read_map(in_file: &str, map: &mut HashMap<String, String>) -> std::io::Result<()> {
    let f = std::fs::File::open(in_file)?;
    let mut s: String = String::new();
    let mut r = std::io::BufReader::new(f);
    r.read_to_string(&mut s)?;
    let scan = rs_lex::rlex::Scanner::new(&s);
    let v: Vec<Box<dyn rs_lex::rlex::Token>> = scan.into_iter().collect();
    let mut index = 3;
    if v.len() > 3
        && v[0].get_string() == "map"
        && v[1].get_string() == "="
        && v[2].get_string() == "{"
    {
        loop {
            if index + 1 < v.len() {
                let s1 = v[index].get_string();
                let s2 = v[index + 2].get_string();
                map.insert(convert_from_slash(s1), convert_from_slash(s2));
                if v[index + 3].get_string() == "}" {
                    break;
                }
                index += 3;
            } else {
                break;
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut map1: HashMap<String, String> = HashMap::new();
    let mut map2: HashMap<String, String> = HashMap::new();
    map1.insert("key1".to_string(), "value1\n\t \" test".to_string());
    map1.insert("key2".to_string(), "value2".to_string());
    save_map("output.txt", &map1)?;
    read_map("output.txt", &mut map2)?;
    for (key, value) in &map2 {
        println!("Key: {}, value: {}", key, value);
    }
    Ok(())
}
