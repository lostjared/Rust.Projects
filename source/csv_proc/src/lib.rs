pub mod csv {

   use rs_lex::rlex::*;
   use std::fs::read_to_string;
   use std::io::Result;

   pub struct Csv {
       pub data: Vec<Vec<String>>,
   }

   impl Csv {
       pub fn new() -> Csv {
           Csv { data: Vec::new() }
       }

       pub fn load_file(&mut self, filename: &str) -> Result<()> {
           let data = read_to_string(filename)?;
           let all_tokens = self.tokenize_data(&data);
           self.convert_tokens_to_data(all_tokens);
           Ok(())
       }

       fn tokenize_data(&self, data: &str) -> Vec<Vec<Box<dyn Token>>> {
           let mut all_tokens = Vec::new();
           for line in data.lines() {
               let mut tokens: Vec<Box<dyn Token>> = Vec::new();
               let mut rlex = Scanner::new(line);
               loop {
                   let token = rlex.scan_token();
                   match token {
                       ScanResult::Error => {
                           eprintln!("Scanner Error on line: {}", line);
                           break;
                       }
                       ScanResult::Ok(tok) => match tok {
                           Some(i) => tokens.push(i),
                           None => break,
                       },
                   }
               }
               all_tokens.push(tokens);
           }
           all_tokens
       }

       fn convert_tokens_to_data(&mut self, all_tokens: Vec<Vec<Box<dyn Token>>>) {
           for row in all_tokens {
               let mut vrow: Vec<String> = Vec::new();
               let mut index = 0;
               'main: loop {
                   if index < row.len() {
                       let id = row[index].get_type();
                       if matches!(id, TokenType::Identifier | TokenType::Digits | TokenType::Symbol | TokenType::String | TokenType::SingleString) {
                           vrow.push(row[index].get_string());
                           index += 1;
                           if index < row.len() && row[index].get_string() == "," {
                               index += 1;
                               continue 'main;
                           } else {
                               break 'main;
                           }
                       } else {
                           break 'main;
                       }
                   } else {
                       break 'main;
                   }
               }
               self.data.push(vrow);
           }
       }
   }
}
