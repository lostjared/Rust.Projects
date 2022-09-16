pub mod rlex {

    use std::collections::HashMap;

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
        pub fn curchar(&self) -> Option<char> {
            self.data.chars().nth(self.pos)
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
        Space,
        Char,
        Identifier,
        String,
        Digits,
        Symbol,
    }

    pub trait Token {
        fn get_type(&self) -> TokenType;
        fn set_type(&mut self, t: TokenType);
        fn get_string(&self) -> String;
    }

    pub struct TokenValue {
        token: String,
        token_type: TokenType,
    }

    impl Token for TokenValue {
        fn get_type(&self) -> TokenType {
            self.token_type
        }
        fn set_type(&mut self, t: TokenType) {
            self.token_type = t;
        }
        fn get_string(&self) -> String {
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
        token_map: HashMap<char, TokenType>
    }

    impl Scanner {
        pub fn new(input: &str) -> Self {
            let mut map : HashMap<char, TokenType> = HashMap::new();
            for i in 'a'..='z' {
                map.insert(i, TokenType::Char);
            }
            for i in 'A'..='Z' {
                map.insert(i, TokenType::Char);
            }
            for i in '0'..='9' {
                map.insert(i, TokenType::Digits);
            }
            map.insert('\"', TokenType::String);
            map.insert('\'', TokenType::String);
            map.insert(' ', TokenType::Space);
            map.insert('\n', TokenType::Space);
            map.insert('\t', TokenType::Space);
            map.insert('\r', TokenType::Space);
            let symbols = String::from("~!@#$%^&*()_-+=[]{}<>.,|\\/?;:`");
            for i in symbols.chars() {
                map.insert(i, TokenType::Symbol);
            }
            Self {
                stream: StringStream::new(input),
                token_map: map
            }
        }

        pub fn valid(&self) -> bool {
            if self.stream.pos < self.stream.data.len() {
                return true;
            }
            false
        }

        pub fn type_from_char(&self, c: char) -> Option<TokenType> {
            if self.token_map.contains_key(&c) {
                return Some(self.token_map[&c]);
            }
            println!("{} not found!", c);
            None
        }

        pub fn grab_id(&mut self) -> TokenValue {
            let mut token_string = String::new();
            let token_type = TokenType::Identifier;
            token_string.push(self.stream.getchar().unwrap());
            loop {
                let ch_t = self.stream.getchar();
                match ch_t {
                    Some(ch) => {
                        let ch_type = self.type_from_char(ch).unwrap();
                        match ch_type {
                            TokenType::Char => {
                                token_string.push(ch);
                            }
                            _ => { break; }
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
            TokenValue::new(&token_string, token_type)
        }

        pub fn scan_token(&mut self) -> Option<Box<dyn Token>> {
            let c = self.stream.curchar();
            match c {
                Some(ch) => {
                    let val = self.type_from_char(ch).unwrap();
                    match val {
                        TokenType::Char => {
                            let token = self.grab_id();
                            return Some(Box::new(token));
                        }
                        TokenType::Digits => {

                        }
                        TokenType::String => {

                        }
                        _ => { println!("type: {:?}", val); }
                    }
                }
                None => {

                }
            }
            None
        }
    }

}