use rs_lex::rlex::tree::btree::*;
use rs_lex::rlex::*;
use std::io::BufRead;
use std::io::Write;

#[test]
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
#[derive(Copy, Clone, Debug)]
pub enum Code {
    Add,
    Sub,
    Mul,
    Div,
    Push,
}

#[derive(Copy, Clone, Debug)]
pub struct Instruct {
    pub opcode: Code,
    pub operand1: f64,
    pub operand2: f64,
}

impl Instruct {
    pub fn new(opc: Code, op1: f64, op2: f64) -> Self {
        Self {
            opcode: opc,
            operand1: op1,
            operand2: op2,
        }
    }
}

struct ICode {
    icode: Vec<Instruct>,
}

impl ICode {
    pub fn new() -> Self {
        Self { icode: Vec::new() }
    }
    pub fn push_code(&mut self, ins: Instruct) {
        self.icode.push(ins);
    }

    pub fn translate_code(&self) {
        for i in &self.icode {
            match i.opcode {
                Code::Add => {
                    println!("ADD");
                }
                Code::Sub => {
                    println!("SUB");
                }
                Code::Mul => {
                    println!("MUL");
                }
                Code::Div => {
                    println!("DIV");
                }
                Code::Push => {
                    println!("PUSH {}", i.operand1);
                }
            }
        }
    }
}

fn process_text() {
    let r = std::io::stdin().lock();
    print!("> ");
    std::io::stdout().lock().flush().expect("on flush");
    for line in r.lines() {
        match line {
            Ok(s) => {
                convert_text(&s);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
        print!("> ");
        std::io::stdout().lock().flush().expect("on flush");
    }
}

fn convert_text(input: &str) {
    let mut scan: Scanner = Scanner::new(input);
    let result = scan.collect_lex();
    match result {
        ScanResult::Error => {
            eprintln!("Scanner Error ");
        }
        ScanResult::Ok(tokens) => {
            let mut index: usize = 0;
            let node = expr(false, &tokens, &mut index);
            //TreeNode::print_nodes(&node);
            let mut code: ICode = ICode::new();
            gen(&node, &mut code);
            code.translate_code();
        }
    }
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

fn gen(node: &Node<Ast>, code: &mut ICode) -> f64 {
    match node {
        Some(node) => match node.data.op {
            '+' => {
                let value = gen(&node.left, code) + gen(&node.right, code);
                code.push_code(Instruct::new(Code::Add, 0.0, 0.0));
                return value;
            }
            '-' => {
                let value = gen(&node.left, code) - gen(&node.right, code);
                code.push_code(Instruct::new(Code::Sub, 0.0, 0.0));
                return value;
            }
            '*' => {
                let value = gen(&node.left, code) * gen(&node.right, code);
                code.push_code(Instruct::new(Code::Mul, 0.0, 0.0));
                return value;
            }
            '/' => {
                let value = gen(&node.left, code) / gen(&node.right, code);
                code.push_code(Instruct::new(Code::Div, 0.0, 0.0));
                return value;
            }
            '0' => {
                code.push_code(Instruct::new(Code::Push, node.data.val, 0.0));
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
