pub mod config {

    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    #[derive(Debug)]
    pub struct Config {
        config: HashMap<String, HashMap<String, String>>,
        filename: String,
    }

    impl Config {
        pub fn create(filename: &str) -> Config {
            let tmap: HashMap<String, HashMap<String, String>> = HashMap::new();
            Config {
                config: tmap,
                filename: String::from(filename),
            }
        }

        // should not contain [] or =
        pub fn setkey(&mut self, id: &str, key: &str, value: &str) {
            if key.find('[') != None
                || key.find(']') != None
                || key.find('=') != None
                || key.find('\n') != None
                || key.find('\r') != None
            {
                panic!("Invalid key contains invalid character");
            }
            if value.find('[') != None
                || value.find(']') != None
                || value.find('=') != None
                || value.find('\n') != None
                || value.find('\r') != None
            {
                panic!("Invalid value contains invalid character");
            }
            let v = self
                .config
                .entry(String::from(id))
                .or_insert_with(HashMap::new);
            v.insert(String::from(key), String::from(value));
        }

        // should not contain [] or =
        pub fn getkey(&mut self, id: &str, key: &str) -> Option<String> {
            if self.config.contains_key(id) {
                let v = self.config.entry(String::from(id)).or_default();
                if v.contains_key(key) {
                    let value = v.get(key);
                    Some(String::from(value.unwrap()))
                } else {
                    None
                }
            } else {
                 None
            }
        }

        pub fn save(&self) {
            let mut f = File::create(&self.filename).expect("on create");
            for (id, value) in &self.config {
                writeln!(&mut f, "[{}]", id).expect("error on write");
                for (key, v) in value {
                    writeln!(&mut f, "{}={}", key, v).expect("error on write");
                }
            }
        }

        pub fn load(&mut self) {
            let con = fs::read_to_string(&self.filename);
            match con {
                Ok(contents) => {
                    let val: Vec<&str> = contents.lines().collect();
                    let mut id = String::new();
                    for i in &val {
                        let pos = i.find('[');
                        if pos != None {
                            let pos2 = i.find(']');
                            if pos2 != None {
                                id = String::from(&i[pos.unwrap() + 1..pos2.unwrap()]);
                            }
                        } else {
                            let eq = i.find('=');
                            if eq != None {
                                let eq_u = eq.unwrap();
                                let left = &i[0..eq_u];
                                let right = &i[eq_u + 1..];
                                self.setkey(
                                    &String::from(&id),
                                    &String::from(left),
                                    &String::from(right),
                                );
                            }
                        }
                    }
                }
                Err(e) => { println!("Error: {}", e); }
            }
        }
    }
}
