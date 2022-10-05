
use rs_lex::rlex::*;
use rs_lex::rlex::token_stream::stream::TokenStream;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
  
    for i in std::io::stdin().lock().lines() {
        match i {
            Ok(input) => {
                let mut token_stream = TokenStream::new(Scanner::new(&input));
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