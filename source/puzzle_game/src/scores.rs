pub mod high_scores {

    use std::fs;
    use std::fs::File;
    use std::io::Write;

    pub struct ScoreMenu {
        pub scores: Vec<(String, u32)>,
        pub input: String,
    }

    impl ScoreMenu {
        pub fn new() -> ScoreMenu {
            ScoreMenu {
                scores: Vec::new(),
                input: String::new(),
            }
        }

        pub fn load(&mut self) {
            let contents1 = fs::read_to_string("./img/score.dat");
            match contents1 {
                Ok(contents) => {
                    if contents.len() > 0 {
                        for i in contents.lines() {
                            let pos = i.find(":");
                            if pos == None {
                                continue;
                            }
                            let p = pos.unwrap();
                            let name = &i[0..p];
                            let score = &i[p + 1..];
                            let hscore = score.trim().parse::<u32>();
                            match hscore {
                                Ok(val) => {
                                    self.scores.push((String::from(name), val));
                                }
                                Err(val) => {
                                    println!("Error: {}", val);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
            self.sort_scores();
        }

        pub fn sort_scores(&mut self) {
            self.scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            self.scores.reverse();
        }

        pub fn save(&mut self) {
            let mut cfile = File::create("./img/score.dat").expect("Error creating file");
            for i in &self.scores {
                writeln!(&mut cfile, "{}:{}", i.0, i.1).expect("error on write");
            }
        }

        pub fn type_key(&mut self, key: &str) {
            self.input.push_str(key);
        }
    }
}
