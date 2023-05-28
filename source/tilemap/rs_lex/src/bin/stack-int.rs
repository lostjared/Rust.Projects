// test application for the Scanner
use rs_lex::rlex::*;
use std::io::Read;

#[test]
fn test_stack_math() {
    let mut stack: Vec<Input> = Vec::new();
    scan(&mut stack, "2 2 4 + +");
    assert_eq!(stack.pop(), Some(Input::Digit(8)));
    scan(&mut stack, "-1 -1 +");
    assert_eq!(stack.pop(), Some(Input::Digit(-2)));
    scan(&mut stack, "2 2 2 * *");
    assert_eq!(stack.pop(), Some(Input::Digit(8)));
    scan(&mut stack, "4 1 /");
    assert_eq!(stack.pop(), Some(Input::Digit(0)));
    scan(&mut stack, "1 1 1 - -");
    assert_eq!(stack.pop(), Some(Input::Digit(-1)));
}

/// input enum
#[derive(Debug, PartialEq)]
pub enum Input {
    Digit(i64),
    Operator(char),
}
/// scan
fn scan(stack: &mut Vec<Input>, input: &str) {
    let scan = Scanner::new(input);
    let v: Vec<Box<dyn Token>> = scan.into_iter().collect();
    push_digits(&v, stack);
    print_stack(stack);
}
/// print the stack to stdout
fn print_stack(stack: &[Input]) {
    for (i, item) in stack.iter().enumerate() {
        match item {
            Input::Digit(num) => {
                println!("stack [{}] = {}", i, num);
            }
            Input::Operator(ch) => {
                println!("stack operator [{}]", ch);
            }
        }
    }
}
/// push digits/operations into the stack
fn push_digits(v: &Vec<Box<dyn Token>>, stack: &mut Vec<Input>) {
    let mut index: usize = 0;

    while index < v.len() {
        if index + 1 < v.len()
            && v[index].get_type() == TokenType::Symbol
            && v[index].get_string() == "-"
            && v[index + 1].get_type() == TokenType::Digits
        {
            index += 1;
            let d: i64 = v[index].get_string().parse().unwrap();
            let num = Input::Digit(-d);
            index += 1;
            stack.push(num);
        } else if v[index].get_type() == TokenType::Digits {
            stack.push(Input::Digit(v[index].get_string().parse().unwrap()));
            index += 1;
        } else if v[index].get_type() == TokenType::Symbol {
            match v[index].get_string().chars().next().unwrap() {
                '+' => {
                    let left = stack.pop().unwrap();
                    let right = stack.pop().unwrap();
                    let mut d1: i64 = 0;
                    let mut d2: i64 = 0;
                    if let Input::Digit(num) = left {
                        d1 = num;
                    }
                    if let Input::Digit(num) = right {
                        d2 = num;
                    }
                    stack.push(Input::Digit(d1 + d2));
                }
                '-' => {
                    let left = stack.pop().unwrap();
                    let right = stack.pop().unwrap();
                    let mut d1: i64 = 0;
                    let mut d2: i64 = 0;
                    if let Input::Digit(num) = left {
                        d1 = num;
                    }
                    if let Input::Digit(num) = right {
                        d2 = num;
                    }

                    stack.push(Input::Digit(d1 - d2));
                }
                '*' => {
                    let left = stack.pop().unwrap();
                    let right = stack.pop().unwrap();
                    let mut d1: i64 = 0;
                    let mut d2: i64 = 0;
                    if let Input::Digit(num) = left {
                        d1 = num;
                    }
                    if let Input::Digit(num) = right {
                        d2 = num;
                    }

                    stack.push(Input::Digit(d1 * d2));
                }
                '/' => {
                    let left = stack.pop().unwrap();
                    let right = stack.pop().unwrap();
                    let mut d1: i64 = 0;
                    let mut d2: i64 = 0;
                    if let Input::Digit(num) = left {
                        d1 = num;
                    }
                    if let Input::Digit(num) = right {
                        d2 = num;
                    }
                    stack.push(Input::Digit(d1 / d2));
                }
                _ => {
                    panic!("Unsupported operator ");
                }
            }
            index += 1;
        } else {
            index += 1;
        }
    }
}

/// read data
fn read_data() {
    let mut r = std::io::stdin().lock();
    let mut s = String::new();
    r.read_to_string(&mut s).expect("read to string");
    let mut stack: Vec<Input> = Vec::new();
    scan(&mut stack, &s);
}

/// main function
fn main() -> std::io::Result<()> {
    read_data();
    Ok(())
}
