/// validate function decleration statement
/// function test();

use rs_lex::rlex::token_stream::stream::TokenStream;
use rs_lex::rlex::*;
use std::io::BufRead;

fn validate(input: &str) -> bool {
    let mut stream = TokenStream::create(Scanner::new(input)).unwrap();
    define(&mut stream);
    true
}

fn define(stream: &mut TokenStream) {
    stream.consume_token_string("function");
    stream.consume_token(TokenType::Identifier);
    stream.consume_token_string("(");
    stream.consume_token_string(")");
    stream.consume_token_string(";");
}

fn main() -> std::io::Result<()> {
    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(data) => {
                if !data.trim().is_empty() && validate(&data) {
                    println!("Correct");
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}
