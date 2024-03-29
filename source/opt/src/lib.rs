//! Get command line arguments

/// module
pub mod argz {

    /// this function process a vector of command line arguments using string input and closeure F(char,String)
    pub fn getopt<F: FnMut(char, String)>(args: &Vec<String>, input: &str, mut f: F) -> u32 {
        let mut arg_count = 0;
        let mut i = 1;
        while i < args.len() {
            let arg = args.get(i).unwrap();
            let pos = arg.find('-');
            if pos != None {
                for z in pos.unwrap() + 1..arg.len() {
                    let ch = arg.chars().nth(z).unwrap();
                    let fch = input.find(ch);
                    if fch != None {
                        let fchx = fch.unwrap();
                        let ch2 = input.chars().nth(fchx + 1);
                        if ch2 != None && ch2.unwrap() == ':' {
                            let next = args.get(i + 1);
                            if next == None {
                                eprintln!("Error required arugment for {} not found", ch);
                                continue;
                            }
                            let n = next.unwrap();
                            if n.find('-') != None {
                                eprintln!("Error required argument for {} not found", ch);
                                continue;
                            }
                            f(ch, String::from(n));
                            arg_count += 1;
                            i += 1;
                        } else {
                            f(ch, String::new());
                            arg_count += 1;
                        }
                    } else {
                        println!("Unrecongized argument: {}", ch);
                    }
                }
                i += 1;
            } else {
                f('-', String::from(arg));
                i += 1;
            }
        }
        arg_count
    }
}
