use rs_lex::rlex::token_stream::stream::TokenStream;
use rs_lex::rlex::*;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    for i in std::io::stdin().lock().lines() {
        match i {
            Ok(input) => {
                let mut token_stream = TokenStream::create(Scanner::new(&input)).unwrap();
                while token_stream.valid() {
                    println!("{}", token_stream.current().get_string());
                    token_stream.next_token();
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}
