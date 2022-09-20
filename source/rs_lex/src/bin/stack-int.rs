use rs_lex::rlex::*;
use std::io::Read;

#[test]
fn test_stack_math() {
    let mut stack: Vec<Input> = Vec::new();
    scan(&mut stack, "2 2 4 + +");
    assert_eq!(stack.pop(), Some(Input::Digit(8)));
}

#[derive(Debug, PartialEq)]
pub enum Input {
    Digit(u64),
    Operator(char),
}

fn scan(stack: &mut Vec<Input>, input: &str) {
    let scan = Scanner::new(input);
    let v: Vec<Box<dyn Token>> = scan.into_iter().collect();
    let mut index: usize = 0;
    push_digits(&v, &mut index, stack);
    //print_stack(stack);
}

fn print_stack(stack: &Vec<Input>) {
    for i in 0..stack.len() {
        match stack[i] {
            Input::Digit(num) => {
                println!("stack [{}] = {}", i, num);
            }
            Input::Operator(ch) => {
                println!("stack operator [{}]", ch);
            }
        }
    }
}

fn push_digits(v: &Vec<Box<dyn Token>>, index: &mut usize, stack: &mut Vec<Input>) {
    if *index < v.len() && v[*index].get_type() == TokenType::Digits {
        stack.push(Input::Digit(v[*index].get_string().parse().unwrap()));
        *index += 1;
        push_digits(v, index, stack);
    } else if *index < v.len() && v[*index].get_type() == TokenType::Symbol {
        match v[*index].get_string().chars().nth(0).unwrap() {
            '+' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let mut d1: u64 = 0;
                let mut d2: u64 = 0;
                match left {
                    Input::Digit(num) => {
                        d1 = num;
                    }
                    _ => {}
                }
                match right {
                    Input::Digit(num) => {
                        d2 = num;
                    }
                    _ => {}
                }
                stack.push(Input::Digit(d1 + d2));
            }
            '-' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let mut d1: u64 = 0;
                let mut d2: u64 = 0;
                match left {
                    Input::Digit(num) => {
                        d1 = num;
                    }
                    _ => {}
                }
                match right {
                    Input::Digit(num) => {
                        d2 = num;
                    }
                    _ => {}
                }
                stack.push(Input::Digit(d1 - d2));
            }
            '*' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let mut d1: u64 = 0;
                let mut d2: u64 = 0;
                match left {
                    Input::Digit(num) => {
                        d1 = num;
                    }
                    _ => {}
                }
                match right {
                    Input::Digit(num) => {
                        d2 = num;
                    }
                    _ => {}
                }
                stack.push(Input::Digit(d1 * d2));
            }
            '/' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let mut d1: u64 = 0;
                let mut d2: u64 = 0;
                match left {
                    Input::Digit(num) => {
                        d1 = num;
                    }
                    _ => {}
                }
                match right {
                    Input::Digit(num) => {
                        d2 = num;
                    }
                    _ => {}
                }
                stack.push(Input::Digit(d1 / d2));
            }
            _ => {
                panic!("Unsupported operator ");
            }
        }
        *index += 1;
        push_digits(v, index, stack);
    }
}

fn read_data() {
    let mut r = std::io::stdin().lock();
    let mut s = String::new();
    r.read_to_string(&mut s).expect("read to string");
    let mut stack: Vec<Input> = Vec::new();
    scan(&mut stack, &s);
}

fn main() -> std::io::Result<()> {
    read_data();
    Ok(())
}
