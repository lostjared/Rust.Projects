use rs_lex::rlex::*;
use std::io::Read;


#[test]
fn test_parse() {
    assert_eq!(evaluate("2+2"), 4.0);
    assert_eq!(evaluate("2+2*4"), 10.0);
    assert_eq!(evaluate("(2+2)*4"), 16.0);
}

fn evaluate(input: &str) -> f64 {
    let scan = Scanner::new(input);
    let tokens: Vec<Box<dyn Token>> = scan.into_iter().collect();
    let mut index: usize = 0;
    expr(false, &tokens, &mut index)
}

fn parse_expr() {
    let mut r = std::io::stdin().lock();
    let mut s = String::new();
    r.read_to_string(&mut s).expect("Error reading data");
    let scan = Scanner::new(&s);
    let tokens: Vec<Box<dyn Token>> = scan.into_iter().collect();
    let mut index: usize = 0;
    println!("value is: {}", expr(false, &tokens, &mut index));
}

fn expr(get: bool, tokens: &Vec<Box<dyn Token>>, index: &mut usize) -> f64 {
    let mut left: f64 = term(get, tokens, index);
    while *index < tokens.len() {
        match tokens[*index].get_type() {
            TokenType::Symbol => {
                match tokens[*index].get_string().chars().nth(0).unwrap() {
                    '+' => {
                        let t = term(true, tokens, index);
                        left += t;
                    }
                    '-' => {
                        let t = term(true, tokens, index);
                        left -= t;
                    }
                    _ => {
                        return left;
                    }
                }
            }
            _ => {
                return left;
            }
        }

    }
    left
}

fn term(get: bool, tokens: &Vec<Box<dyn Token>>, index: &mut usize) -> f64 {
    let mut left: f64 = prim(get, tokens, index);
    while *index < tokens.len() {
        match tokens[*index].get_type() {
            TokenType::Symbol => {
                match tokens[*index].get_string().chars().nth(0).unwrap() {
                    '*' => {
                        let t = prim(true, tokens, index);
                        left *= t;
                    }
                    '/' => {
                        let t = prim(true, tokens, index);
                        if t == 0.0 {
                            panic!("Divide by zero");
                        }
                        left /= t;
                    }
                    _ => {
                        return left;
                    }
                }
            }
            _ => { return left; }
        }
    }
    left
}

fn prim(get: bool, tokens: &Vec<Box<dyn Token>>, index: &mut usize) -> f64 {
    if get {
        *index += 1;
    }
    match tokens[*index].get_type() {
        TokenType::Digits => {
            let d: f64 = tokens[*index].get_string().parse().unwrap();
            *index += 1;
            return d;
        }
        TokenType::Identifier => {}
        TokenType::Symbol => match tokens[*index].get_string().chars().nth(0).unwrap() {
            '-' => {
                return -prim(true, tokens, index);
            }
            '(' => {
                let f = expr(true, tokens, index);
                *index += 1;
                return f;
            }
            _ => {}
        },
        _ => {}
    }
    0.0
}

fn main() -> std::io::Result<()> {
    parse_expr();
    Ok(())
}
