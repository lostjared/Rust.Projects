pub mod stream {
    use crate::rlex::*;

    pub struct TokenStream {
        pub tokens: Vec<Box<dyn Token>>,
        pub index: usize,
    }

    impl TokenStream {
        pub fn new(scanner: Scanner) -> Self {
            let tokens: Vec<Box<dyn Token>> = scanner.into_iter().collect();

            Self {
                tokens: tokens,
                index: 0,
            }
        }

        pub fn create(mut scanner: Scanner) -> Option<Self> {
            let result = scanner.collect_lex();
            match result {
                ScanResult::Error => {
                    return None;
                }
                ScanResult::Ok(tokenz) => {
                    return Some(Self {
                        tokens: tokenz,
                        index: 0
                    });
                }
            }
        }

        pub fn valid(&self) -> bool {
            self.index < self.tokens.len()
        }

        pub fn current(&self) -> &Box<dyn Token> {
            &self.tokens[self.index]
        }

        pub fn next_token(&mut self) {
            if self.index < self.tokens.len() {
                self.index += 1;
            }
        }

        pub fn match_token(&self, t: TokenType) -> bool {
            if self.current().get_type() == t {
                return true;
            }
            false
        }

        pub fn match_token_string(&self, input: &str) -> bool {
            if self.current().get_string() == input.to_string() {
                return true;
            }
            false
        }

        pub fn consume_token(&mut self, t1: TokenType) {
            let t2 = self.current().get_type();
            if t1 == t2 {
                self.next_token();
            } else {
                panic!("Expected token: {:?} found {:?}", t1, t2);
            }
        }

        pub fn consume_token_string(&mut self, input: &str) {
            let st = self.current().get_string();
            if st == input.to_string() {
                self.next_token();
            } else {
                panic!("Expected: {} found {}", input, st);
            }
        }
    }
}
