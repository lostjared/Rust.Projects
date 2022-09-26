use rs_lex::rlex::*;
use std::io::BufRead;

#[test]
fn test_collect_tokens() {
    assert_eq!(collect_tokens_count("x = 1"), 3);
    assert_eq!(collect_tokens_count("x = x + x"), 5);
    assert_eq!(collect_tokens_count("print(\"Hello World\");"), 5);
    assert_eq!(collect_tokens_member("test(&x);"), 6);
}

/// collect tokens count (how many?)
fn collect_tokens_count(input: &str) -> usize {
    let mut scan = Scanner::new(input);
    let token_result = collect_tokens(&mut scan);
    match token_result {
        ScanResult::Error => {
            eprintln!("Error quitting");
            0
        }
        ScanResult::Ok(tokens) => tokens.len(),
    }
}
/// collect tokens count (How many)?
fn collect_tokens_member(input: &str) -> usize {
    let mut scan = Scanner::new(input);
    let token_result = scan.collect_lex();
    match token_result {
        ScanResult::Error => {
            eprintln!("Error quitting");
            0
        }
        ScanResult::Ok(tokens) => tokens.len(),
    }
}

/// main function
fn main() -> std::io::Result<()> {
    for i in std::io::stdin().lock().lines() {
        match i {
            Ok(line) => {
                let length = collect_tokens_count(&line);
                println!("Count: {}", length);
                let length = collect_tokens_member(&line);
                println!("Count: {}", length);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
    Ok(())
}
