use rs_lex::rlex::tree::btree::*;
use rs_lex::rlex::*;
use std::io::BufRead;

fn tree_test() {
    let mut tree: Tree<i32> = Tree::new();
    let mut node: TreeNode<i32> = TreeNode::new(25);
    node.left = Some(Box::new(TreeNode::new(15)));
    let node_left = Some(Box::new(TreeNode::new(100)));
    let node_right = Some(Box::new(TreeNode::new(200)));
    node.right = Some(Box::new(TreeNode::new_node(15, node_left, node_right)));
    tree.root = Some(Box::new(node));
    //TreeNode::<i32>::print_nodes(&tree.root);
    tree.print_nodes();
}

#[test]
fn test_tree() {
    tree_test();
}

#[derive(Copy, Clone, Debug)]
pub struct Ast {
    pub op: char,
    pub val: f64,
}

impl Ast {
    pub fn new(o: char) -> Self {
        Self { op: o, val: 0.0 }
    }

    pub fn new_val(v: f64) -> Self {
        Self { op: '0', val: v }
    }

    pub fn new_value(v: f64) -> Node<Self> {
        Some(Box::new(TreeNode::new(Self::new_val(v))))
    }
}

fn process_text() {
    let r = std::io::stdin().lock();
    for line in r.lines() {
        match line {
            Ok(s) => {
                convert_text(&s);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}

fn convert_text(input: &str) {
    let scan: Scanner = Scanner::new(input);
    let tokens: Vec<TokenVar> = scan.into_iter().collect();
    let mut index: usize = 0;
    let node = expr(false, &tokens, &mut index);
    TreeNode::print_nodes(&node);
    println!("Value is: {}", eval(&node));
}

fn expr(get: bool, tokens: &Vec<TokenVar>, index: &mut usize) -> Node<Ast> {
    let mut left = term(get, tokens, index);

    while *index < tokens.len() {
        match tokens[*index].get_type() {
            TokenType::Symbol => match tokens[*index].get_string().chars().next().unwrap() {
                '+' => {
                    let t = term(true, tokens, index);
                    left = Some(Box::new(TreeNode::new_node(Ast::new('+'), left, t)));
                }
                '-' => {
                    let t = term(true, tokens, index);
                    left = Some(Box::new(TreeNode::new_node(Ast::new('-'), left, t)));
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

fn term(get: bool, tokens: &Vec<TokenVar>, index: &mut usize) -> Node<Ast> {
    let mut left = prim(get, tokens, index);
    while *index < tokens.len() {
        match tokens[*index].get_type() {
            TokenType::Symbol => match tokens[*index].get_string().chars().next().unwrap() {
                '*' => {
                    let t = prim(true, tokens, index);
                    left = Some(Box::new(TreeNode::new_node(Ast::new('*'), left, t)));
                }
                '/' => {
                    let t = prim(true, tokens, index);
                    left = Some(Box::new(TreeNode::new_node(Ast::new('/'), left, t)));
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

fn prim(get: bool, tokens: &Vec<TokenVar>, index: &mut usize) -> Node<Ast> {
    if get {
        *index += 1;
    }
    match tokens[*index].get_type() {
        TokenType::Digits => {
            let d: f64 = tokens[*index].get_string().parse().unwrap();
            *index += 1;
            println!("PUSH {}", d);
            return Ast::new_value(d);
        }
        TokenType::Identifier => {}
        TokenType::Symbol => match tokens[*index].get_string().chars().next().unwrap() {
            '-' => {
                let node = prim(true, tokens, index);
                match node {
                    Some(n) => {
                        return Ast::new_value(-n.data.val);
                    }
                    None => {}
                }
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

    Ast::new_value(0.0)
}

fn eval(node: &Node<Ast>) -> f64 {
    match node {
        Some(node) => match node.data.op {
            '+' => {
                return eval(&node.left) + eval(&node.right);
            }
            '-' => {
                return eval(&node.left) - eval(&node.right);
            }
            '*' => {
                return eval(&node.left) * eval(&node.right);
            }
            '/' => {
                return eval(&node.left) / eval(&node.right);
            }
            '0' => {
                return node.data.val;
            }
            _ => {}
        },
        None => {}
    }
    0.0
}

fn main() -> std::io::Result<()> {
    process_text();
    Ok(())
}
