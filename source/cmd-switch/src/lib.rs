
pub mod cmd_sw {

    use std::collections::HashMap;

    pub struct Argument {
        pub key: String,
        pub value: String,
        pub desc: String
    }

    impl Argument {
        fn new(k: &str, v: &str, d: &str) -> Self {
            Argument {
                key: String::from(k),
                value: String::from(v),
                desc: String::from(d),
            }
        }
    }

    pub fn print_accepted_args(desc: &HashMap<String, String>) {
        println!("Accepted Arguments:");
        for (key, value) in desc {
            println!("\t--{} [{}]", key, value);
        }
        print!("\n");
    }

    pub fn print_accepted_args_map(desc: &HashMap<String, Argument>) {
        println!("Accepted Arguments:");
        for (key, value) in desc {
            println!("\t--{} [{}]", key, value.desc);
        }
        print!("\n");
    }

    pub fn parse_args(args: &Vec<String>, desc: &HashMap<String, String>) -> HashMap<String, Argument> {
        let mut argz : HashMap<String, Argument> = HashMap::new();
        for i in args.into_iter().skip(1) {
            let pos = i.find("=");
            let pos_s = i.find("--");
            if pos != None && pos_s != None {
                let loc = (pos_s.unwrap(), pos.unwrap());
                let key = &i[loc.0+2..loc.1];
                let right = &i[loc.1+1..i.len()];
                let d;
                let s = desc.get(key);
                if s != None {
                    d = String::from(s.unwrap());
                } else {
                    d = String::from("None");
                }
                argz.insert(String::from(key), Argument::new(key,right,&d));
            } else {
                //println!("Incorrect format: use --key=value found: {}", i);
            }
        }
        argz
    }

    pub fn parse_args_require(args: &Vec<String>, desc: &HashMap<String, (String, bool)>) -> HashMap<String, Argument> {
        let mut argz : HashMap<String, Argument> = HashMap::new();
        let mut arg_req : HashMap<String, bool> = HashMap::new();
        for i in args.into_iter().skip(1) {
            let pos = i.find("=");
            let pos_s = i.find("--");
            if pos != None && pos_s != None {
                let loc = (pos_s.unwrap(), pos.unwrap());
                let key = &i[loc.0+2..loc.1];
                let right = &i[loc.1+1..i.len()];
                let d;
                let s = desc.get(key);
                if s != None {
                    let val = s.unwrap();
                    d = String::from(&val.0);
                    let d2 = String::from(key);
                    arg_req.insert(d2, true);
                } else {
                    d = String::from("None");
                }
                argz.insert(String::from(key), Argument::new(key,right,&d));
            } else {
                //println!("Incorrect format: use --key=value found: {}", i);
            }
        }

        for (key, value) in desc {
            if value.1 == true {
                if !arg_req.contains_key(key) {
                    panic!("Error required argument {} missing: [{}]\n", key, value.0);
                } 
            }
        }
        argz
    }
}