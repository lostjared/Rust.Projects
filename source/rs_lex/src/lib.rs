/// rlex moudle
pub mod rlex {

    use std::collections::HashMap;

    pub mod map;
    pub mod tree;

    /// StringStream struct to keep track of current character position
    #[derive(Clone, Debug)]
    pub struct StringStream {
        data: String,
        pos: usize,
        lineno: usize,
    }

    impl StringStream {
        /// create a new string stream
        pub fn new(input: &str) -> Self {
            let mut s = String::from(input);
            s.push('\n');
            Self {
                data: s,
                pos: 0,
                lineno: 1,
            }
        }
        /// get a character from stream
        pub fn getchar(&mut self) -> Option<char> {
            if self.pos < self.data.len() {
                let c = self.data.chars().nth(self.pos);
                self.pos += 1;
                if c == Some('\n') {
                    self.lineno += 1;
                }
                return c;
            }
            None
        }
        /// current character in stream
        pub fn curchar(&self) -> Option<char> {
            self.data.chars().nth(self.pos)
        }
        /// previous character from stream
        pub fn prevchar(&self) -> Option<char> {
            self.data.chars().nth(self.pos - 1)
        }

        /// put back a character into stream
        pub fn putback(&mut self) {
            if self.pos > 0 {
                self.pos -= 1;
            }
        }
        /// peek next character from stream without increasing position
        pub fn peekchar(&mut self) -> Option<char> {
            if self.pos + 1 < self.data.len() {
                return self.data.chars().nth(self.pos + 1);
            }
            None
        }
        /// advance position without getting character
        pub fn advance(&mut self) {
            self.getchar();
        }

        /// advance position by number of iterations
        pub fn advance_by(&mut self, index: usize) {
            for _i in 0..index {
                self.getchar();
            }
        }

        /// move position back by number of iterations
        pub fn moveback_by(mut self, index: usize) {
            self.pos -= index;
        }

        /// rewind to start of stream
        pub fn rewind(&mut self) {
            self.pos = 0;
            self.lineno = 1;
        }
        /// reset stream with new input string
        pub fn reset(&mut self, input: &str) {
            self.data = input.to_string();
            self.pos = 0;
            self.lineno = 1;
        }
        /// set stream position
        pub fn set_pos(&mut self, p: usize) {
            self.pos = p;
        }
    }

    /// the different types of tokens
    #[derive(Copy, Clone, Debug, PartialEq)]
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

    /// Token type trait
    pub trait Token {
        fn get_type(&self) -> TokenType;
        fn set_type(&mut self, t: TokenType);
        fn get_string(&self) -> String;
        fn get_line(&self) -> usize;
    }

    /// Token value
    #[derive(PartialEq)]
    pub struct TokenValue {
        token: String,
        token_type: TokenType,
        line_number: usize,
    }

    pub type TokenVar = Box<dyn Token>;

    impl Token for TokenValue {
        /// get token type
        fn get_type(&self) -> TokenType {
            self.token_type
        }
        /// set token type
        fn set_type(&mut self, t: TokenType) {
            self.token_type = t;
        }
        /// get token string
        fn get_string(&self) -> String {
            self.token.to_owned()
        }
        /// get token line position
        fn get_line(&self) -> usize {
            self.line_number
        }
    }

    impl TokenValue {
        /// create a new token value
        pub fn new(input: String, t: TokenType, lineno: usize) -> Self {
            Self {
                token: input,
                token_type: t,
                line_number: lineno,
            }
        }
    }

    /// the scanner structure
    pub struct Scanner {
        stream: StringStream,
        token_map: HashMap<char, TokenType>,
        oper: Vec<String>,
    }

    /// scan result enum for success or error
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum ScanResult<T> {
        Ok(T),
        Error,
    }

    impl Scanner {
        /// create a new Scanner struct
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
            let symbols = String::from("~!@#$%^&*()-+=[]{}<>.,|\\/?;:`");
            for i in symbols.chars() {
                map.insert(i, TokenType::Symbol);
            }
            map.insert('_', TokenType::Char);
            let o: Vec<&str> = vec![
                "++", "--", ">>", "<<", ".=", "+=", "-=", "*=", "/=", "<>", "!=", "<=", ">=", "==",
                "&&", "||", "^=", "%=", "&=", "?=", "->", "=>", "::", "**", ":=", "***", "|=",
                "..", "===", "!==", ">>=", "<<=",
            ];
            let mut o_s: Vec<String> = Vec::new();
            for i in &o {
                o_s.push(i.to_string());
            }
            Self {
                stream: StringStream::new(input),
                token_map: map,
                oper: o_s,
            }
        }

        /// is scanner structure valid?
        pub fn valid(&self) -> bool {
            if self.stream.pos < self.stream.data.len() {
                return true;
            }
            false
        }

        /// convert type from character
        pub fn type_from_char(&self, c: char) -> Option<TokenType> {
            if self.token_map.contains_key(&c) {
                return Some(self.token_map[&c]);
            }
            println!("{} not found!", c);
            None
        }

        /// grab id token value
        pub fn grab_id(&mut self) -> TokenValue {
            let mut token_string = String::new();
            let token_type = TokenType::Identifier;
            let lineno = self.stream.lineno;
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
                            TokenType::String | TokenType::Symbol | TokenType::SingleString => {
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
            TokenValue::new(token_string, token_type, lineno)
        }

        /// grab digits token value
        pub fn grab_digits(&mut self) -> Option<TokenValue> {
            let mut token_string = String::new();
            let token_type = TokenType::Digits;
            let lineno = self.stream.lineno;
            token_string.push(self.stream.getchar().unwrap());
            let mut dot_count = 0;
            loop {
                let ch_t = self.stream.getchar();
                match ch_t {
                    Some(ch) => {
                        let ch_type = self.type_from_char(ch).unwrap();
                        match ch_type {
                            TokenType::Digits => {
                                token_string.push(ch);
                            }
                            TokenType::String | TokenType::SingleString => {
                                self.stream.putback();
                                break;
                            }
                            TokenType::Symbol => {
                                if ch == '.' {
                                    dot_count += 1;
                                    if dot_count > 1 && self.stream.prevchar() == Some('.') {
                                        self.stream.putback();
                                        self.stream.putback();
                                        token_string.remove(token_string.len() - 1);
                                        break;
                                    } else {
                                        token_string.push(ch);
                                    }
                                } else {
                                    self.stream.putback();
                                    break;
                                }
                            }
                            TokenType::Space
                            | TokenType::NULL
                            | TokenType::Identifier
                            | TokenType::Char => {
                                break;
                            }
                        }
                    }
                    None => {
                        break;
                    }
                }
            }

            if token_string.chars().nth(token_string.len() - 1) == Some('.') {
                eprintln!(
                    "Lexer Error: Invalid decimal point on Number on Line: {}",
                    self.stream.lineno
                );
                return None;
            }

            Some(TokenValue::new(token_string, token_type, lineno))
        }

        /// grab string token value
        pub fn grab_string(&mut self) -> Option<TokenValue> {
            let mut token_string = String::new();
            let token_type = TokenType::String;
            let lineno = self.stream.lineno;
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
                        eprintln!("Lexer Error: String missing closing \" quote");
                        return None;
                    }
                }
            }
            Some(TokenValue::new(token_string, token_type, lineno))
        }

        /// grab single quote string token value
        pub fn grab_single_string(&mut self) -> Option<TokenValue> {
            let mut token_string = String::new();
            let token_type = TokenType::SingleString;
            let lineno = self.stream.lineno;
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
                        eprintln!("Lexer Error: String missing closing \' quote.");
                        return None;
                    }
                }
            }
            Some(TokenValue::new(token_string, token_type, lineno))
        }

        /// grab symbol token value
        pub fn grab_symbol(&mut self) -> TokenValue {
            let mut token_string = String::new();
            let token_type = TokenType::Symbol;
            let lineno = self.stream.lineno;
            let ch = self.stream.getchar().unwrap();
            let ch2 = self.stream.curchar().unwrap();
            let ch3 = self.stream.peekchar();
            let mut found = false;

            if ch3 != None {
                let mut cmp_str = String::new();
                cmp_str.push(ch);
                cmp_str.push(ch2);
                cmp_str.push(ch3.unwrap());
                for i in &self.oper {
                    if *i == cmp_str {
                        token_string.push_str(&cmp_str);
                        self.stream.advance_by(2);
                        found = true;
                    }
                }
            }
            if !found {
                let mut cmp_str = String::new();
                cmp_str.push(ch);
                cmp_str.push(ch2);
                for i in &self.oper {
                    if *i == cmp_str {
                        token_string.push_str(&cmp_str);
                        self.stream.advance();
                    }
                }
            }
            if token_string.is_empty() {
                token_string.push(ch);
            }
            TokenValue::new(token_string, token_type, lineno)
        }

        /// scan for token function
        pub fn scan_token(&mut self) -> ScanResult<Option<Box<dyn Token>>> {
            let c = self.stream.curchar();
            match c {
                Some(ch) => {
                    let val = self.type_from_char(ch).unwrap();
                    match val {
                        TokenType::Char => {
                            let token = self.grab_id();
                            return ScanResult::Ok(Some(Box::new(token)));
                        }
                        TokenType::Digits => {
                            let token = self.grab_digits();
                            if token == None {
                                return ScanResult::Error; // Error occoured stop scan
                            }
                            return ScanResult::Ok(Some(Box::new(token.unwrap())));
                        }
                        TokenType::String => {
                            let token = self.grab_string();
                            if token == None {
                                return ScanResult::Error; // Error occoured stop scan
                            }
                            return ScanResult::Ok(Some(Box::new(token.unwrap())));
                        }
                        TokenType::SingleString => {
                            let token = self.grab_single_string();
                            if token == None {
                                return ScanResult::Error; // Error occoured stop scan
                            }
                            return ScanResult::Ok(Some(Box::new(token.unwrap())));
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
                            return ScanResult::Ok(Some(Box::new(token)));
                        }
                        TokenType::Space => {
                            self.stream.advance();
                            return self.scan_token();
                        }
                        TokenType::NULL => {
                            self.stream.advance();
                            println!("Unrecongnized character: {}", ch);
                            return self.scan_token();
                        }
                        _ => {
                            println!("type: {:?}", val);
                        }
                    }
                }
                None => {}
            }
            ScanResult::Ok(None)
        }

        /// collect all tokens return as Vector
        pub fn collect_lex(&mut self) -> ScanResult<Vec<Box<dyn Token>>> {
            collect_tokens(self)
        }
    }

    impl Iterator for Scanner {
        type Item = Box<dyn Token>;
        /// next item for iterator
        fn next(&mut self) -> Option<Self::Item> {
            let x = self.scan_token();
            match x {
                ScanResult::Ok(it) => it,
                ScanResult::Error => None,
            }
        }
    }

    /// convert to slash format
    pub fn convert_to_slash(input: &String) -> String {
        let mut s = String::new();
        let mut i = 0;
        while i < input.len() {
            let ch = input.chars().nth(i).unwrap();
            i += 1;
            match ch {
                '\\' => {
                    let chx = input.chars().nth(i).unwrap();
                    s.push(ch);
                    s.push(chx);
                    i += 1;
                }
                '\"' => {
                    s.push_str("\\\"");
                }
                '\'' => {
                    s.push_str("\\\'");
                }
                '\n' => {
                    s.push_str("\\n");
                }
                '\r' => {
                    s.push_str("\\r");
                }
                '\t' => {
                    s.push_str("\\t");
                }
                _ => {
                    s.push(ch);
                }
            }
        }
        s
    }
    /// convert from slash format
    pub fn convert_from_slash(input: &String) -> String {
        let mut s: String = String::new();
        let mut i = 0;
        while i < input.len() {
            let ch = input.chars().nth(i).unwrap();
            i += 1;
            match ch {
                '\\' => {
                    let chx = input.chars().nth(i).unwrap();
                    match chx {
                        'n' => {
                            s.push('\n');
                        }
                        't' => {
                            s.push('\t');
                        }
                        'r' => {
                            s.push('\r');
                        }
                        '\"' => {
                            s.push('\"');
                        }
                        '\'' => {
                            s.push('\'');
                        }
                        _ => {
                            s.push(chx);
                        }
                    }
                    i += 1;
                }
                _ => {
                    s.push(ch);
                }
            }
        }
        s
    }

    /// consume token (if doesn't exisit panic)
    pub fn consume_token(v: &[Box<dyn Token>], index: &mut usize, tok: &str) {
        if v[*index].get_string() == *tok {
            *index += 1;
        } else {
            panic!("Expected: {} found {}", tok, v[*index].get_string());
        }
    }
    /// match token
    pub fn match_token(v: &[Box<dyn Token>], index: usize, tok: &str) -> bool {
        v[index].get_string() == *tok
    }
    /// match token increase position
    pub fn match_token_inc(v: &[Box<dyn Token>], index: &mut usize, tok: &str) -> bool {
        if v[*index].get_string() == *tok {
            *index += 1;
            true
        } else {
            false
        }
    }
    /// match token type return String
    pub fn match_token_type(
        v: &[Box<dyn Token>],
        index: &mut usize,
        tok_t: TokenType,
    ) -> Option<String> {
        if v[*index].get_type() == tok_t {
            let t = v[*index].get_string();
            *index += 1;
            return Some(t);
        }
        None
    }

    /// collect all tokens return as Vector
    pub fn collect_tokens(scan: &mut Scanner) -> ScanResult<Vec<Box<dyn Token>>> {
        let mut v: Vec<Box<dyn Token>> = Vec::new();
        loop {
            let token = scan.scan_token();
            match token {
                ScanResult::Error => {
                    eprintln!("Scanner Error ");
                    return ScanResult::Error;
                }
                ScanResult::Ok(tok) => match tok {
                    Some(i) => {
                        v.push(i);
                    }
                    None => {
                        break;
                    }
                },
            }
        }
        ScanResult::Ok(v)
    }
}
