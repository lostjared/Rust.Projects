pub mod cmd_sw {

    use std::collections::HashMap;

    pub struct Argument {
        pub key: String,
        pub value: String,
        pub desc: String,
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
        println!();
    }

    pub fn print_accepted_args_map(desc: &HashMap<String, Argument>) {
        println!("Accepted Arguments:");
        for (key, value) in desc {
            println!("\t--{} [{}]", key, value.desc);
        }
        println!();
    }

    pub fn print_accepted_args_map_require(desc: &HashMap<String, (String, bool)>) {
        println!("Accepted Arguments:");
        for (key, value) in desc {
            println!("\t--{} [{}] required: {}", key, value.0, value.1);
        }
        println!();
    }

    pub fn parse_args(
        args: &[String],
        desc: &HashMap<String, String>,
    ) -> HashMap<String, Argument> {
        let mut argz: HashMap<String, Argument> = HashMap::new();
        for i in args.iter().skip(1) {
            let pos = i.find('=');
            let mut pos_s = i.find("--");
            let mut pos_f = false;
            if pos_s == None {
                pos_s = i.find('-');
                pos_f = true;
            }
            if pos != None && pos_s != None {
                let loc = (pos_s.unwrap(), pos.unwrap());
                let key;
                if !pos_f {
                    key = &i[loc.0 + 2..loc.1];
                } else {
                    key = &i[loc.0 + 1..loc.1];
                }
                let right = &i[loc.1 + 1..];
                let d;
                let s = desc.get(key);
                if s != None {
                    d = String::from(s.unwrap());
                } else {
                    d = String::from("None");
                }
                argz.insert(String::from(key), Argument::new(key, right, &d));
            } else if pos_s != None && pos == None {
                let loc = pos_s.unwrap();
                let k;
                if !pos_f {
                    k = &i[loc + 2..];
                } else {
                    k = &i[loc + 1..];
                }
                let d;
                let s = desc.get(k);
                if s != None {
                    d = String::from(s.unwrap());
                } else {
                    d = String::from("None");
                }
                argz.insert(String::from(k), Argument::new(k, "", &d));
                //println!("Incorrect format: use --key=value found: {}", i);
            }
        }
        argz
    }

    pub fn parse_args_require(
        args: &[String],
        desc: &HashMap<String, (String, bool)>,
    ) -> HashMap<String, Argument> {
        let mut argz: HashMap<String, Argument> = HashMap::new();
        let mut arg_req: HashMap<String, bool> = HashMap::new();
        for i in args.iter().skip(1) {
            let pos = i.find('=');
            let mut pos_s = i.find("--");
            let mut pos_f = false;
            if pos_s == None {
                pos_s = i.find('-');
                pos_f = true;
            }
            if pos != None && pos_s != None {
                let loc = (pos_s.unwrap(), pos.unwrap());
                let key;
                if !pos_f {
                    key = &i[loc.0 + 2..loc.1];
                } else {
                    key = &i[loc.0 + 1..loc.1];
                }
                let right = &i[loc.1 + 1..];
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
                argz.insert(String::from(key), Argument::new(key, right, &d));
            } else if pos_s != None && pos == None {
                let loc = pos_s.unwrap();
                let k;
                if !pos_f {
                    k = &i[loc + 2..];
                } else {
                    k = &i[loc + 1..];
                }
                let v = desc.get(k);
                let d;
                if v != None {
                    let s = v.unwrap();
                    let string_value = String::from(k);
                    d = String::from(&s.0);
                    arg_req.insert(string_value, true);
                } else {
                    d = String::from("None");
                }
                argz.insert(String::from(k), Argument::new(k, "", &d));
                //println!("Incorrect format: use --key=value found: {}", i);
            }
        }

        for (key, value) in desc {
            if value.1 && !arg_req.contains_key(key) {
                panic!("Error required argument {} missing: [{}]\n", key, value.0);
            }
        }
        argz
    }
}
