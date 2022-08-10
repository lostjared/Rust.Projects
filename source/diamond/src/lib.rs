// like perls <> operator
pub mod dmd {

    use std::io::BufRead;

    pub struct Diamond {
        files: Vec<String>,
        cur_index: usize,
        file_open: std::io::BufReader<std::fs::File>,
    }

    impl Diamond {
        pub fn new(args: &Vec<String>) -> Self {
            Diamond {
                files: args.to_owned(),
                cur_index: 0,
                file_open: std::io::BufReader::new(
                    std::fs::File::open(args.get(1).unwrap()).unwrap(),
                ),
            }
        }

        pub fn read_next(&mut self) -> Option<String> {
            let mut line: String = String::new();
            let file_len = self.file_open.read_line(&mut line).expect("on read");
            if file_len > 0 {
                return Some(line);
            }
            self.cur_index += 1;
            if self.cur_index > self.files.len() - 1 {
                return None;
            }
            self.file_open = std::io::BufReader::new(
                std::fs::File::open(self.files.get(self.cur_index).unwrap()).unwrap(),
            );
            self.read_next()
        }
    }

    impl Iterator for Diamond {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> {
            self.read_next()
        }
    }
}
