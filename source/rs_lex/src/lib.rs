pub mod rlex {

    #[derive(Clone, Debug)]
    pub struct StringStream {
        data: String,
        pos: usize,
    }

    impl StringStream {
        pub fn new(input: &str) -> Self {
            Self {
                data: input.to_string(),
                pos: 0,
            }
        }
        pub fn getchar(&mut self) -> Option<char> {
            if self.pos < self.data.len() {
                let c = self.data.chars().nth(self.pos).unwrap();
                self.pos += 1;
                return Some(c);
            }
            None
        }
        pub fn putback(&mut self) {
            if self.pos > 0 {
                self.pos -= 1;
            }
        }
        pub fn peekchar(&mut self) -> Option<char> {
            if self.pos + 1 < self.data.len() {
                return Some(self.data.chars().nth(self.pos + 1).unwrap());
            }
            None
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub enum TokenType {
        NULL,
        Identifier,
        String,
        Digits,
        Symbol,
    }

    pub trait Token {
        fn getType(&self) -> TokenType;
        fn setType(&mut self, t: TokenType);
        fn getString(&self) -> String;
    }

    pub struct TokenValue {
        token: String,
        token_type: TokenType,
    }

    impl Token for TokenValue {
        fn getType(&self) -> TokenType {
            self.token_type
        }
        fn setType(&mut self, t: TokenType) {
            self.token_type = t;
        }
        fn getString(&self) -> String {
            self.token.to_owned()
        }
    }

    impl TokenValue {
        pub fn new(input: &str, t: TokenType) -> Self {
            Self {
                token: input.to_string(),
                token_type: t
            }
        }
    }

    pub struct Scanner {
        stream: StringStream,
    }

    impl Scanner {
        pub fn new(input: &str) -> Self {
            Self {
                stream: StringStream::new(input)
            }
        }

        pub fn valid(&self) -> bool {
            if self.stream.pos < self.stream.data.len() {
                return true;
            }
            false
        }

        pub fn scan_token(&mut self) -> Box<dyn Token> {
            let token = TokenValue::new("test", TokenType::NULL);
            Box::new(token)
        }
    }

}
