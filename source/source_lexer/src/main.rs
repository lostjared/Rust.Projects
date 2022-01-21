// practice learning rust
#![allow(unreachable_code)]

use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use rustyline::error::ReadlineError;
use rustyline::Editor;

mod lexer {

    pub struct Source {
        data: String,
        index: usize,
        pub line: usize,
    }

    #[derive(Eq, PartialEq, Debug)]
    pub enum TokenType {
        NULL,
        ID,
        STRING,
        NUMBER,
        OP,
        SPACE,
    }

    pub trait CharacterType {
        fn char_to_type(&self, index: usize) -> TokenType;
    }

    impl CharacterType for String {
        fn char_to_type(&self, index: usize) -> TokenType {
            let val = self.chars().nth(index).unwrap();
            match val {
                'a'..='z' | 'A'..='Z' => {
                    return TokenType::ID;
                }
                '0'..='9' => {
                    return TokenType::NUMBER;
                }
                ' ' | '\t' | '\r' | '\n' => {
                    return TokenType::SPACE;
                }
                '\"' => {
                    return TokenType::STRING;
                }
                '!' | '@' | '#' | '$' | '%' | '^' | '&' | '|' | '*' | '(' | ')' | '{' | '}'
                | '+' | '=' | '-' | '<' | '>' | '.' | ',' | ';' | '[' | ']' | '/' | '\\' => {
                    return TokenType::OP;
                }
                _ => {
                    return TokenType::NULL;
                }
            }
            TokenType::NULL
        }
    }

    #[test]
    fn type_from_char() {
        let s = String::from("T!");
        assert_eq!(s.char_to_type(0), TokenType::ID);
        assert_eq!(s.char_to_type(1), TokenType::OP);
    }

    impl TokenType {
        pub fn type_to_string(self) -> String {
            let value = match self {
                TokenType::ID => String::from("Identifier"),
                TokenType::NUMBER => String::from("Digits"),
                TokenType::STRING => String::from("String"),
                TokenType::SPACE => String::from("Space"),
                TokenType::OP => String::from("Operator"),
                TokenType::NULL => String::from("NULL"),
            };
            value
        }
    }

    impl Source {
        pub fn type_from_char(val: &char) -> TokenType {
            match val {
                'a'..='z' | 'A'..='Z' => {
                    return TokenType::ID;
                }
                '0'..='9' => {
                    return TokenType::NUMBER;
                }
                ' ' | '\t' | '\r' | '\n' => {
                    return TokenType::SPACE;
                }
                '\"' => {
                    return TokenType::STRING;
                }
                '!' | '@' | '#' | '$' | '%' | '^' | '&' | '|' | '*' | '(' | ')' | '{' | '}'
                | '+' | '=' | '-' | '<' | '>' | '.' | ',' | ';' | '[' | ']' | '/' | '\\' => {
                    return TokenType::OP;
                }
                _ => {
                    return TokenType::NULL;
                }
            }
            TokenType::NULL
        }

        pub fn new(s: String) -> Source {
            Source {
                data: s,
                index: 0,
                line: 1,
            }
        }

        pub fn getchar(&mut self) -> char {
            let i: usize = self.index;
            if i + 1 >= self.data.len() {
                return 0 as char;
            }
            let ch = self.data.chars().nth(i).unwrap();
            self.index += 1;
            ch
        }

        pub fn peekchar(&mut self) -> char {
            let i: usize = self.index;
            if i + 1 >= self.data.len() {
                return 0 as char;
            }
            let ch = self.data.chars().nth(i).unwrap();
            ch
        }

        fn grab_id(&mut self) -> String {
            let mut token: String = String::new();
            loop {
                let ch = self.getchar();
                match Source::type_from_char(&ch) {
                    TokenType::ID => {
                        token.push(ch);
                    }
                    TokenType::NUMBER => {
                        token.push(ch);
                    }
                    TokenType::NULL => {
                        return token;
                    }
                    _ => {
                        self.index -= 1;
                        return token;
                    }
                }
            }
            token
        }

        fn grab_number(&mut self) -> String {
            let mut token: String = String::new();
            let mut counter = 0;
            loop {
                let ch = self.getchar();
                match Source::type_from_char(&ch) {
                    TokenType::NUMBER => {
                        token.push(ch);
                    }
                    TokenType::OP => {
                        if counter == 0 && ch == '.' {
                            token.push(ch);
                            counter += 1;
                            continue;
                        } else if counter == 1 && ch == '.' {
                            println!(
                                "Syntax Error: Line: {}, Too many decimal points.\n",
                                self.line
                            );
                            std::process::exit(-1);
                        } else {
                            self.index -= 1;
                            return token;
                        }
                    }
                    _ => {
                        return token;
                    }
                }
            }

            token
        }

        fn grab_op(&mut self) -> String {
            let mut token: String = String::new();
            loop {
                let ch = self.getchar();

                match Source::type_from_char(&ch) {
                    TokenType::OP => {
                        let c = self.peekchar();
                        if c == '=' {
                            token.push(ch);
                            token.push(c);
                            self.index += 1;
                            return token;
                        } else {
                            let c = self.peekchar();
                            if ch == c {
                                token.push(ch);
                                token.push(c);
                                self.index += 1;
                                return token;
                            } else {
                                token.push(ch);
                                return token;
                            }
                        }
                    }
                    _ => {
                        return token;
                    }
                }
            }
            token
        }

        fn grab_string(&mut self) -> String {
            let mut token: String = String::new();
            self.index += 1;
            loop {
                let ch = self.getchar();
                if ch == 0 as char {
                    return token;
                }
                match Source::type_from_char(&ch) {
                    TokenType::STRING | TokenType::NULL => return token,
                    TokenType::OP => {
                        if ch == '\\' {
                            match self.getchar() {
                                'n' => token.push('\n'),
                                't' => token.push('\t'),
                                'r' => token.push('\r'),
                                _ => {}
                            }
                        } else {
                            token.push(ch);
                        }
                    }
                    _ => {
                        token.push(ch);
                    }
                }
            }
            token
        }

        pub fn lex(&mut self, output: &mut String) -> TokenType {
            loop {
                let ch = self.getchar();
                if ch == 0 as char {
                    return TokenType::NULL;
                }
                let t = Source::type_from_char(&ch);
                match t {
                    TokenType::ID => {
                        // grab id
                        self.index -= 1;
                        *output = self.grab_id();
                        return TokenType::ID;
                    }
                    TokenType::NUMBER => {
                        self.index -= 1;
                        *output = self.grab_number();
                        return TokenType::NUMBER;
                    }
                    TokenType::OP => {
                        if ch == '/' {
                            let mut c = self.getchar();
                            if c == '/' {
                                // comment
                                while c != '\n' && c != 0 as char {
                                    c = self.getchar();
                                }
                                if c == 0 as char {
                                    return TokenType::NULL;
                                }
                            } else {
                                self.index -= 1;
                            }
                        }
                        self.index -= 1;
                        *output = self.grab_op();
                        return TokenType::OP;
                    }
                    TokenType::STRING => {
                        self.index -= 1;
                        *output = self.grab_string();
                        return TokenType::STRING;
                    }
                    TokenType::SPACE => {
                        if ch == '\n' {
                            self.line += 1;
                        }
                        continue;
                    }
                    TokenType::NULL => {
                        continue;
                    }
                }
            }
            // TokenType::NULL
        }
    }

    pub struct Token {
        pub line: usize,
        pub token: TokenType,
        pub value: String,
    }
}

#[test]
fn test_lex_id() {
    let s = String::from("test");
    let mut val = lexer::Source::new(s);
    let mut x: String = String::new();
    let t = val.lex(&mut x);
    assert_eq!(t, lexer::TokenType::ID);
}

#[test]
fn test_type() {
    assert_eq!(lexer::Source::type_from_char(&'a'), lexer::TokenType::ID);
    assert_eq!(lexer::Source::type_from_char(&'A'), lexer::TokenType::ID);
    assert_eq!(lexer::Source::type_from_char(&'+'), lexer::TokenType::OP);
    assert_eq!(
        lexer::Source::type_from_char(&'1'),
        lexer::TokenType::NUMBER
    );
}

#[test]
fn test_lex() {
    let s = "test 123 ++".to_string();
    let mut val = lexer::Source::new(s);
    let mut x: String = String::new();
    let mut t = val.lex(&mut x);
    assert_eq!(t, lexer::TokenType::ID);
    t = val.lex(&mut x);
    assert_eq!(t, lexer::TokenType::NUMBER);
    t = val.lex(&mut x);
    assert_eq!(t, lexer::TokenType::OP);
}

fn proc_lex_list(input: String) -> Option<Vec<lexer::Token>> {
    let mut v = Vec::new();
    let mut val = lexer::Source::new(input);
    loop {
        let mut input = String::new();
        let tok = val.lex(&mut input);
        if tok == lexer::TokenType::NULL {
            if v.len() == 0 {
                return None;
            } else {
                return Some(v);
            }
        }
        let t = lexer::Token {
            line: val.line,
            token: tok,
            value: input,
        };
        v.push(t);
    }
    None
}

#[test]
fn test_list() {
    let t = proc_lex_list("test 123 +".to_string());
    match t {
        Some(val) => {
            assert_eq!(val.len(), 2);
            let v = val.get(0).unwrap();
            assert_eq!(v.token, lexer::TokenType::ID);
        }
        None => {}
    }
}

fn proc_lex(input: String) -> usize {

    if input.trim().eq("quit") {
        return 0;
    }
    let mut val = lexer::Source::new(input);

    loop {
        let mut input = String::new();
        let tok = val.lex(&mut input);
        match tok {
            lexer::TokenType::ID
            | lexer::TokenType::NUMBER
            | lexer::TokenType::OP
            | lexer::TokenType::STRING => {


                println!(
                    "{} - [{}] - Line: {}",
                    input,
                    tok.type_to_string(),
                    val.line
                );
            }
            _ => {
                break;
            }
        }
    }
    1
}

fn convert_to_html(input: &String) -> String {
    let mut x: usize = 0;
    let mut s = String::new();
    while x < input.len() {
        let val = input.chars().nth(x).unwrap();
        match val {
            '<' => {
                s.push_str("&lt;");
            }
            '>' => {
                s.push_str("&gt;");
            }
            ' ' => {
                s.push_str("&nbsp;");
            }
            '\n' => {
                s.push_str("<br>");
            }
            '\t' => {
                s.push_str("&nbsp;&nbsp;&nbsp;&nbsp;");
            }
            _ => {
                s.push(val);
            }
        }
        x += 1;
    }
    s
}

fn proc_lex_output(input: &String, output: &String) {
    let mut val = lexer::Source::new(String::from(input));
    let mut cfile = File::create(output).expect("Error creating file");
    writeln!(&mut cfile, "<!doctype html><head><title>{} Lex Output Table</title></head><body><table border=\"1\" cellpadding=\"1\" cellspacing=\"1\"><tr><th>Line</th><th>Token</th><th>Type</th></tr>", output).expect("error on write");
    loop {
        let mut input = String::new();
        let tok = val.lex(&mut input);
        match tok {
            lexer::TokenType::ID
            | lexer::TokenType::NUMBER
            | lexer::TokenType::OP
            | lexer::TokenType::STRING => {
                writeln!(
                    &mut cfile,
                    "<tr><th>{}</th><th>{}</th><th>{}</th></tr>",
                    val.line,
                    convert_to_html(&input),
                    tok.type_to_string()
                )
                .expect("error on write");
            }
            _ => {
                break;
            }
        }
    }
    writeln!(&mut cfile, "</table></body></html>").expect("error on write");
}




fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {

        }
        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    let mut iline = String::from(line);
                    iline.push_str("\n");
                    if proc_lex(iline) == 0 {
                        rl.save_history("history.txt").expect("on save");
                        std::process::exit(0);
                    }
                },
                Err(ReadlineError::Interrupted) => {
                   // println!("CTRL-C");
                    break
                },
                Err(ReadlineError::Eof) => {
                 //   println!("CTRL-D");
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }
    } else if args.len() == 2 {
        let value = args.get(1).unwrap();
        let contents = fs::read_to_string(value).expect("Error reading the file");
        proc_lex(contents);
    } else if args.len() == 3 {
        let value = args.get(1).unwrap();
        let output = args.get(2).unwrap();
        let contents = fs::read_to_string(value).expect("Error reading the file");
        proc_lex_output(&contents, output);
    }
}
