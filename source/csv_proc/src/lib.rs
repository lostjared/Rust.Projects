pub mod csv {

    use rs_lex::rlex::*;

    pub struct Csv {
        pub data: Vec<Vec<String>>,
    }

    impl Csv {
        pub fn new() -> Csv {
            Csv { data: Vec::new() }
        }
        pub fn load_file(&mut self, filename: &str) -> std::io::Result<()> {
            let mut all_tokens = Vec::new();
            let data = std::fs::read_to_string(filename)?;
            for line in data.lines() {
                let mut tokens: Vec<Box<dyn Token>> = Vec::new();
                let mut rlex = rs_lex::rlex::Scanner::new(line);
                loop {
                    let token = rlex.scan_token();
                    match token {
                        ScanResult::Error => {
                            eprintln!("Scanner Error ");
                            break;
                        }
                        ScanResult::Ok(tok) => match tok {
                            Some(i) => {
                                tokens.push(i);
                            }
                            None => {
                                break;
                            }
                        },
                    }
                }
                all_tokens.push(tokens);
            }
            // convert
            for row in &all_tokens {
                let mut index = 0;
                let mut vrow: Vec<String> = Vec::new();
                'main: loop {
                    if index < row.len() {
                        let id = row[index].get_type();
                        if id == TokenType::Identifier
                            || id == TokenType::Digits
                            || id == TokenType::Symbol
                            || id == TokenType::String
                            || id == TokenType::SingleString
                        {
                            let value = row[index].get_string();
                            vrow.push(value);
                            index += 1;
                            if index < row.len() {
                                let value = row[index].get_string();
                                if value == "," {
                                    index += 1;
                                    continue 'main;
                                } else {
                                    break 'main;
                                }
                            }
                        }
                        {
                            break 'main;
                        }
                    } else {
                        break 'main;
                    }
                }
                self.data.push(vrow);
            }
            Ok(())
        }
    }
}
