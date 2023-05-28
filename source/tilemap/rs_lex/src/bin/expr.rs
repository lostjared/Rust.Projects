// parse simple expressions like 2*2+(4*2)
// this is another program to test the Lexer (Scanner)
// run
// $ cargo test

use rs_lex::rlex::*;
use std::io::BufRead;
use std::io::Write;

#[test]
fn test_parse() {
    assert_eq!(evaluate("2.0+2.0"), 2.0 + 2.0);
    assert_eq!(evaluate("2.0+2.0*4.0"), 2.0 + 2.0 * 4.0);
    assert_eq!(evaluate("(2.0+2.0)*4.0"), (2.0 + 2.0) * 4.0);
    assert_eq!(evaluate("((2.0+2.0)*4.0)/2.0"), ((2.0 + 2.0) * 4.0) / 2.0);
    assert_eq!(evaluate("-1.0 + -1.0"), -1.0 + -1.0);
}

/// evaulate expression
fn evaluate(input: &str) -> f64 {
    if input.trim().is_empty() {
        return 0.0;
    }

    let scan = Scanner::new(input);
    let tokens: Vec<Box<dyn Token>> = scan.into_iter().collect();
    let mut index: usize = 0;
    expr(false, &tokens, &mut index)
}

/// parse expression
fn parse_expr() {
    let r = std::io::stdin().lock();
    print!("> ");
    std::io::stdout().lock().flush().expect("on flush");
    for line in r.lines() {
        match line {
            Ok(e) => {
                println!("value is: {}", evaluate(&e));
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        print!("> ");
        std::io::stdout().lock().flush().expect("on flush");
    }
}

/// recursive expression parser
fn expr(get: bool, tokens: &Vec<Box<dyn Token>>, index: &mut usize) -> f64 {
    let mut left: f64 = term(get, tokens, index);
    while *index < tokens.len() {
        match tokens[*index].get_type() {
            TokenType::Symbol => match tokens[*index].get_string().chars().next().unwrap() {
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
            },
            _ => {
                return left;
            }
        }
    }
    left
}

/// term
fn term(get: bool, tokens: &Vec<Box<dyn Token>>, index: &mut usize) -> f64 {
    let mut left: f64 = prim(get, tokens, index);
    while *index < tokens.len() {
        match tokens[*index].get_type() {
            TokenType::Symbol => match tokens[*index].get_string().chars().next().unwrap() {
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
            },
            _ => {
                return left;
            }
        }
    }
    left
}

/// prim
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
        TokenType::Symbol => match tokens[*index].get_string().chars().next().unwrap() {
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

/// main function
fn main() -> std::io::Result<()> {
    parse_expr();
    Ok(())
}
