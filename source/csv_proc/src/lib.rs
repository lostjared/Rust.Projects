

pub mod xcsv {

    use std::io::Write;

    pub type Table = Vec<Vec<String>>;

    pub struct StringScan {
        pub index: usize,
        pub dlen: usize,
        pub data: String,
    }

    impl StringScan {
        pub fn zero() -> StringScan {
            StringScan {
                index: 0,
                dlen: 0,
                data: String::new(),
            }
        }

        pub fn new(string_data: &str) -> StringScan {
            let s = string_data.to_string();
            StringScan {
                index: 0,
                dlen: s.len(),
                data: s,
            }
        }

        pub fn getchar(&mut self) -> Option<char> {
            if self.index < self.dlen {
                let ch = self.data.chars().nth(self.index).unwrap();
                self.index += 1;
                return Some(ch);
            }
            None
        }
        pub fn curchar(&self) -> char {
            self.data.chars().nth(self.index).unwrap()
        }

        pub fn peekchar(&self, lookahead: usize) -> Option<char> {
            if self.index + lookahead < self.dlen {
                return Some(self.data.chars().nth(self.index + lookahead).unwrap());
            }
            None
        }
    }

   pub struct XCsv {
        pub scan: StringScan,
        pub table: Table,
    }

    impl XCsv {
        pub fn new() -> XCsv {
            XCsv {
                scan: StringScan::zero(),
                table: Vec::new(),
            }
        }

        pub fn load_file(&mut self, filename: &str, sep: &char) -> std::io::Result<()> {
            let s = std::fs::read_to_string(filename)?;
            for line in s.lines() {
                self.scan = StringScan::new(&line);
                if let Ok(result) = self.tokenize(sep) {
                    self.table.push(result);
                }
            }
            Ok(())
        }

        pub fn save_file(&mut self, filename: &str, sep: &char) -> std::io::Result<()> {
            let f = std::fs::File::create(filename)?;
            let mut wr = std::io::BufWriter::new(f);
            for row in &self.table {
                let mut row_data = Vec::new();
                for cell in row {
                    let mut cell_str = String::new();
                    let needs_quotes =
                        cell.contains(&sep.to_string()) || cell.contains('"') || cell.contains(' ');

                    if needs_quotes {
                        cell_str.push('"');
                        for ch in cell.chars() {
                            if ch == '"' {
                                cell_str.push('"'); // Escape double quotes
                            }
                            cell_str.push(ch);
                        }
                        cell_str.push('"');
                    } else {
                        cell_str = cell.clone();
                    }

                    if !cell_str.is_empty() {
                        row_data.push(cell_str);
                    }
                }
                writeln!(wr, "{}", row_data.join(&sep.to_string()))?;
            }
            Ok(())
        }

        pub fn add_row(&mut self, data: &[&str]) {
            let data_row: Vec<String> = data.iter().map(|&s| s.to_string()).collect();
            self.table.push(data_row);
        }

        pub fn remove_row(&mut self, row: usize) {
            self.table.remove(row);
        }

        pub fn at(&mut self, row: usize, col: usize) -> &String {
            &self.table[row][col]
        }

        fn not_token(ch: &char, sep: &char) -> bool {
            *ch == ' ' || *ch == '\r' || *ch == '\n' || *ch == '\t' || *ch == *sep
        }

        pub fn grab_token(&mut self, sep: &char) -> Option<String> {
            if self.scan.index >= self.scan.dlen {
                return None;
            }

            match self.scan.curchar() {
                '\"' => {
                    let mut data = String::new();
                    self.scan.getchar();
                    loop {
                        let ch = self.scan.getchar();
                        if let Some(c) = ch {
                            if c == '\"' {
                                if let Some(next_c) = self.scan.peekchar(0) {
                                    if next_c == '\"' {
                                        data.push('\"');
                                        self.scan.getchar();
                                    } else {
                                        break;
                                    }
                                }
                            } else {
                                data.push(c);
                            }
                        } else {
                            break;
                        }
                    }
                    Some(data)
                }
                _ => {
                    let mut data = String::new();
                    loop {
                        let ch = self.scan.getchar();
                        if let Some(c) = ch {
                            if !XCsv::not_token(&c, sep) {
                                data.push(c);
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    Some(data)
                }
            }
        }

        pub fn tokenize(&mut self, sep: &char) -> Result<Vec<String>, String> {
            let mut v: Vec<String> = Vec::new();
            loop {
                if let Some(token) = self.grab_token(sep) {
                    v.push(token);
                } else {
                    break;
                }
                if let Some(c) = self.scan.peekchar(0) {
                    if c == *sep {
                        self.scan.getchar();
                    }
                }
            }
            Ok(v)
        }
    }
}
