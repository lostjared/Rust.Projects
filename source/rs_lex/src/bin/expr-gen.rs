// output steps involved in solving the expression

use rs_lex::rlex::*;
use std::io::BufRead;
use std::io::Write;
use std::collections::HashMap;

/// evalulate expression save variables to vmap
fn evaluate(input: &str, vmap: &mut HashMap<String, f64>) -> f64 {
    if input.trim().is_empty() {
        return 0.0;
    }
    let mut scan = Scanner::new(input);
    let tokens_result = collect_tokens(&mut scan);
    match tokens_result {
        ScanResult::Error => {
            return 0.0;
        }
        ScanResult::Ok(tokens) => {
            if match_token(&tokens, 0, "quit") {
                std::process::exit(0);
            }
            let mut index: usize = 0;
            let value = expr(false, &tokens, &mut index, vmap);
            return value;        
        }
    }
}
/// parse expressions
fn parse_expr() {
    let r = std::io::stdin().lock();
    print!("> ");
    std::io::stdout().lock().flush().expect("on flush");
    let mut vmap : HashMap<String, f64> = HashMap::new();
    for line in r.lines() {
        match line {
            Ok(e) => {
                println!("Value for expression is: {}", evaluate(&e, &mut vmap));
                println!("<--- VAR TABLE --->");
                for (key, value) in &vmap {
                    println!("{:7} -> {}", key, value);
                }
                println!("<--- END TABLE --->");
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        print!("> ");
        std::io::stdout().lock().flush().expect("on flush");
    }
}

/// recursive expression
fn expr(get: bool, tokens: &Vec<Box<dyn Token>>, index: &mut usize, vmap: &mut HashMap<String, f64>) -> f64 {
    let mut left: f64 = term(get, tokens, index, vmap);
    while *index < tokens.len() {
        match tokens[*index].get_type() {
            TokenType::Symbol => match tokens[*index].get_string().chars().nth(0).unwrap() {
                '+' => {
                    let t = term(true, tokens, index, vmap);
                    println!("ADD {} + {}", left, t);
                    left += t;
                }
                '-' => {
                    let t = term(true, tokens, index, vmap);
                    println!("SUB {} - {}", left, t);
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
fn term(get: bool, tokens: &Vec<Box<dyn Token>>, index: &mut usize, vmap: &mut HashMap<String, f64>) -> f64 {
    let mut left: f64 = prim(get, tokens, index, vmap);
    while *index < tokens.len() {
        match tokens[*index].get_type() {
            TokenType::Symbol => match tokens[*index].get_string().chars().nth(0).unwrap() {
                '*' => {
                    let t = prim(true, tokens, index, vmap);
                    println!("MUL {} * {}", left, t);
                    left *= t;
                }
                '/' => {
                    let t = prim(true, tokens, index, vmap);
                    if t == 0.0 {
                        panic!("Divide by zero");
                    }
                    println!("DIV {} / {}", left, t);
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
fn prim(get: bool, tokens: &Vec<Box<dyn Token>>, index: &mut usize, vmap: &mut HashMap<String, f64>) -> f64 {
    if get {
        *index += 1;
    }
    match tokens[*index].get_type() {
        TokenType::Digits => {
            let d: f64 = tokens[*index].get_string().parse().unwrap();
            *index += 1;
            return d;
        }
        TokenType::Identifier => {
            let map_id = tokens[*index].get_string();
            let var_d;
            if vmap.contains_key(&map_id) {
                var_d = vmap[&map_id];
            } else {
                let lineno = tokens[*index].get_line();
                if *index+1 < tokens.len() && tokens[*index+1].get_string() != "=" {
                    panic!("Variable {} not declared on Line: {}", map_id, lineno);
                } else if *index+1 >= tokens.len()  {
                    panic!("Variable {} not declared on Line: {}", map_id, lineno);
                } else {
                    var_d = 0.0;
                }
            }
            *index += 1;
            if *index < tokens.len() && tokens[*index].get_string() == "=" {
                let var_d = expr(true, tokens, index, vmap);
                vmap.insert(map_id.to_owned(), var_d);
                println!("{} EQUALS {}", map_id, var_d);
                return var_d;
            } else  if *index < tokens.len() && tokens[*index].get_string() == "+=" {
                let var_d = expr(true, tokens, index, vmap);
                let mut var_val = if vmap.contains_key(&map_id) { vmap[&map_id] } else { 0.0 };
                println!("{} PLUS-EQUALS {}", map_id, var_d);
                var_val += var_d;
                vmap.insert(map_id.to_owned(), var_val);
                return var_val;
            } else  if *index < tokens.len() && tokens[*index].get_string() == "-=" {
                let var_d = expr(true, tokens, index, vmap);
                let mut var_val = if vmap.contains_key(&map_id) { vmap[&map_id] } else { 0.0 };
                println!("{} MINUS-EQUALS {}", map_id, var_d);
                var_val -= var_d;
                vmap.insert(map_id.to_owned(), var_val);
                return var_val;
            } else  if *index < tokens.len() && tokens[*index].get_string() == "*=" {
                let var_d = expr(true, tokens, index, vmap);
                let mut var_val = if vmap.contains_key(&map_id) { vmap[&map_id] } else { 0.0 };
                println!("{} MUL-EQUALS {}", map_id, var_d);
                var_val *= var_d;
                vmap.insert(map_id.to_owned(), var_val);
                return var_val;
            } else  if *index < tokens.len() && tokens[*index].get_string() == "/=" {
                let var_d = expr(true, tokens, index, vmap);
                let mut var_val = if vmap.contains_key(&map_id) { vmap[&map_id] } else { 0.0 };
                if var_d == 0.0 {
                    panic!("Divde by zero");
                }
                println!("{} DIV-EQUALS {}", map_id, var_d);
                var_val /= var_d;
                vmap.insert(map_id.to_owned(), var_val);
                return var_val;
            } 
            else {
                return var_d;
            }
        }
        TokenType::Symbol => match tokens[*index].get_string().chars().nth(0).unwrap() {
            '-' => {
                return -prim(true, tokens, index, vmap);
            }
            '(' => {
                let f = expr(true, tokens, index, vmap);
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
