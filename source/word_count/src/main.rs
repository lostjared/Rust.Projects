use std::collections::HashMap;
use std::env;
use std::fs;

fn remove_chars(input: &str) -> String {
    let mut output = String::new();
    for i in input.chars() {
        match i {
            'a'..='z' | 'A'..='Z' | ' ' | '\n' => {
                output.push(i);
            }
            _ => {

            }
        }
    }
    output
}

fn count_words(input: String) {
    let inp = input.trim();
    let mut words: HashMap<String, i32> = HashMap::new();
    let val = remove_chars(inp);
    let values = val.split(" ");

    for i in values {
        if i.len() == 0 {
            continue;
        }
        if words.contains_key(i) {
            let val = words[i];
            let value = String::from(i);
            words.insert(value, val + 1);
        } else {
            let value = String::from(i);
            words.insert(value, 1);
        }
    }
    let mut v: Vec<_> = words.into_iter().collect();
    v.sort_by(|x, y| x.0.cmp(&y.0));

    for val in v {
        println!("{}:{}", val.0, val.1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error requires 1 argument file.txt\nword_count file.txt\n");
        return;
    }
    let inputfile = args.get(1).unwrap();
    let contents = fs::read_to_string(inputfile).expect("Error reading the file");
    count_words(contents);
}
