//! High Scores Module
pub mod high_scores {

    use std::fs;
    use std::fs::File;
    use std::io::Write;

    /// high scores menu structure
    pub struct ScoreMenu {
        pub scores: Vec<(String, u32)>,
        pub input: String,
    }

    impl ScoreMenu {
        /// create a new Score Menu
        pub fn new() -> ScoreMenu {
            ScoreMenu {
                scores: Vec::new(),
                input: String::new(),
            }
        }

        /// load the score information from file
        pub fn load(&mut self) {
            let contents1 = fs::read_to_string("./img/score.dat");
            match contents1 {
                Ok(contents) => {
                    if !contents.is_empty() {
                        for i in contents.lines() {
                            let pos = i.find(':');
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

        /// set the scores
        pub fn sort_scores(&mut self) {
            self.scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            self.scores.reverse();
        }
        /// save the score information to file
        pub fn save(&mut self) {
            let mut cfile = File::create("./img/score.dat").expect("Error creating file");
            for i in &self.scores {
                writeln!(&mut cfile, "{}:{}", i.0, i.1).expect("error on write");
            }
        }

        /// process key input for scores menu
        pub fn type_key(&mut self, key: &str) {
            if key.find(':') == None {
                self.input.push_str(key);
            }
        }
    }
}
