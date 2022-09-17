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
                let c = self.data.chars().nth(self.pos);
                self.pos += 1;
                return c;
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
                return self.data.chars().nth(self.pos + 1);
            }
            None
        }
        pub fn advance(&mut self) {
            self.pos += 1;
        }
        pub fn rewind(&mut self) {
            self.pos = 0;
        }
        pub fn reset(&mut self, input: &str) {
            self.data = input.to_string();
            self.pos = 0;
        }
        pub fn set_pos(&mut self, p: usize) {
            self.pos = p;
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub enum TokenType {
        NULL,
        Space,
        Char,
        Identifier,
        String,
        SingleString,
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
                token_type: t,
            }
        }
    }

    pub struct Scanner {
        stream: StringStream,
        token_map: HashMap<char, TokenType>,
    }

    impl Scanner {
        pub fn new(input: &str) -> Self {
            let mut map: HashMap<char, TokenType> = HashMap::new();
            for i in 0..255u8 {
                map.insert(i as char, TokenType::NULL);
            }
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
            map.insert('\'', TokenType::SingleString);
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
                token_map: map,
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
                            TokenType::Char | TokenType::Digits => {
                                token_string.push(ch);
                            }
                            TokenType::String | TokenType::Symbol => {
                                self.stream.putback();
                                break;
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
            TokenValue::new(&token_string, token_type)
        }

        pub fn grab_digits(&mut self) -> TokenValue {
            let mut token_string = String::new();
            let token_type = TokenType::Digits;
            token_string.push(self.stream.getchar().unwrap());
            loop {
                let ch_t = self.stream.getchar();
                match ch_t {
                    Some(ch) => {
                        let ch_type = self.type_from_char(ch).unwrap();
                        match ch_type {
                            TokenType::Digits => {
                                token_string.push(ch);
                            }
                            TokenType::String => {
                                self.stream.putback();
                                break;
                            }
                            TokenType::Symbol => {
                                if ch == '.' {
                                    token_string.push(ch);
                                } else {
                                    self.stream.putback();
                                    break;
                                }
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
            TokenValue::new(&token_string, token_type)
        }

        pub fn grab_string(&mut self) -> TokenValue {
            let mut token_string = String::new();
            let token_type = TokenType::String;
            self.stream.advance();
            loop {
                let ch = self.stream.getchar();
                match ch {
                    Some(ch_v) => {
                        if ch_v == '\\' {
                            token_string.push(ch_v);
                            let chx = self.stream.getchar().unwrap();
                            token_string.push(chx);
                            continue;
                        } else if ch_v == '\"' {
                            break;
                        } else {
                            token_string.push(ch_v);
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
            TokenValue::new(&token_string, token_type)
        }

        pub fn grab_single_string(&mut self) -> TokenValue {
            let mut token_string = String::new();
            let token_type = TokenType::SingleString;
            self.stream.advance();
            loop {
                let ch = self.stream.getchar();
                match ch {
                    Some(ch_v) => {
                        if ch_v == '\\' {
                            token_string.push(ch_v);
                            let chx = self.stream.getchar().unwrap();
                            token_string.push(chx);
                            continue;
                        } else if ch_v == '\'' {
                            break;
                        } else {
                            token_string.push(ch_v);
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
            TokenValue::new(&token_string, token_type)
        }

        pub fn grab_symbol(&mut self) -> TokenValue {
            let mut token_string = String::new();
            let token_type = TokenType::Symbol;
            let oper = vec![
                "++", "--", ">>", "<<", ".=", "+=", "-=", "*=", "/=", "<>", "!=", "<=", ">=", "==",
                "&&", "||", "^=", "%=", "&=", "?=", "->", "=>", "::", "**", "***", "|=",
            ];
            let ch = self.stream.getchar().unwrap();
            let ch2 = self.stream.curchar().unwrap();
            let mut cmp_str = String::new();
            cmp_str.push(ch);
            cmp_str.push(ch2);
            for i in &oper {
                if i.to_string() == cmp_str {
                    token_string.push_str(&cmp_str);
                    self.stream.advance();
                }
            }
            if token_string.len() == 0 {
                token_string.push(ch);
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
                            let token = self.grab_digits();
                            return Some(Box::new(token));
                        }
                        TokenType::String => {
                            let token = self.grab_string();
                            return Some(Box::new(token));
                        }
                        TokenType::SingleString => {
                            let token = self.grab_single_string();
                            return Some(Box::new(token));
                        }
                        TokenType::Symbol => {
                            if ch == '/' {
                                let chz = self.stream.peekchar();
                                match chz {
                                    Some(com) => match com {
                                        '/' => loop {
                                            let ch = self.stream.getchar();
                                            match ch {
                                                Some(ch2) => {
                                                    if ch2 == '\n' {
                                                        return self.scan_token();
                                                    }
                                                }
                                                None => {
                                                    break;
                                                }
                                            }
                                        },
                                        '*' => {
                                            self.stream.advance();
                                            loop {
                                                let chx = self.stream.getchar();
                                                let ch_close = self.stream.curchar();
                                                if chx != None
                                                    && ch_close != None
                                                    && chx == Some('*')
                                                    && ch_close == Some('/')
                                                {
                                                    self.stream.advance();
                                                    return self.scan_token();
                                                }
                                                if chx == None {
                                                    self.stream.advance();
                                                    return self.scan_token();
                                                }
                                            }
                                        }
                                        _ => {}
                                    },
                                    None => {}
                                }
                            }
                            let token = self.grab_symbol();
                            return Some(Box::new(token));
                        }
                        TokenType::Space => {
                            self.stream.advance();
                            return self.scan_token();
                        }
                        TokenType::NULL => {
                            self.stream.advance();
                            println!("Unrecongized character: {}", ch);
                            return self.scan_token();
                        }
                        _ => {
                            println!("type: {:?}", val);
                        }
                    }
                }
                None => {}
            }
            None
        }
    }

    impl Iterator for Scanner {
        type Item = Box<dyn Token>;
        /// next item for iterator
        fn next(&mut self) -> Option<Self::Item> {
            self.scan_token()
        }
    }
}
