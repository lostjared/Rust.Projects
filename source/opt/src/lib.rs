pub mod argz {

    pub fn getopt<F: Fn(char, String)>(args: &Vec<String>, input: &str, f: F) -> u32 {
        let mut arg_count = 0;
        for i in 1..args.len() {
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
                            let next = args.get(i + 1).unwrap();
                            f(ch, String::from(next));
                            arg_count += 1;
                        } else {
                            f(ch, String::new());
                            arg_count += 1;
                        }
                    } else {
                        println!("Unrecongized argument: {}", ch);
                    }
                }
            }
        }
        arg_count
    }
}
