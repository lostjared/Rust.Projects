use rs_lex::rlex::*;
use std::io::BufRead;


#[test]
fn test_collect_tokens() {
    assert_eq!(collect_tokens_count("x = 1"), 3);
    assert_eq!(collect_tokens_count("x = x + x"), 5);
    assert_eq!(collect_tokens_count("print(\"Hello World\");"), 5);
}

fn collect_tokens_count(input: &str) -> usize {
    let mut scan = Scanner::new(input);
    let token_result = collect_tokens(&mut scan);
    match token_result {
        ScanResult::Error => {
            eprintln!("Error quitting");
            return 0;
        }
        ScanResult::Ok(tokens) => {
            return tokens.len();
        }
    }
}


fn main() -> std::io::Result<()> {
    for i in std::io::stdin().lock().lines() {
        match i {
            Ok(line) => {
                let length = collect_tokens_count(&line);
                println!("Count: {}", length);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
    Ok(())
}