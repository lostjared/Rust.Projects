
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
                desc: String::from(d)
            }
        }
    }

    pub fn parse_args(args: &Vec<String>) -> HashMap<String, Argument> {
        let mut argz : HashMap<String, Argument> = HashMap::new();
        for i in args {
            let pos = i.find("=");
            let pos_s = i.find("--");
            if pos != None && pos_s != None {
                let loc = (pos_s.unwrap(), pos.unwrap());
                let key = &i[loc.0+2..loc.1];
                let right = &i[loc.1+1..i.len()];
                argz.insert(String::from(key), Argument::new(key,right,"description"));
            } 
        }
        argz
    }
}