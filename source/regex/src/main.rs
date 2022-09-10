
use regex::Regex;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let r = Regex::new("[a-z][A-Z]*").unwrap();
    let reader = std::io::stdin().lock();
    for i in reader.lines() {
        match i {
            Ok(line) => {
                if r.is_match(&line) {
                    println!("Matches");
                } else {
                    println!("Does not match");
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
    Ok(())
}