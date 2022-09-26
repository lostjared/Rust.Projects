pub mod rs_map {

    use crate::rlex::{
        consume_token, convert_from_slash, convert_to_slash, match_token, match_token_type,
        Scanner, Token, TokenType,
    };
    use std::collections::{BTreeMap, HashMap};
    use std::io::Read;
    use std::io::Write;

    #[test]
    fn test_scan_map() {
        let s = "map = {\n\"value1\" = \"value2\"\n}\n";
        let scan = Scanner::new(s);
        let v: Vec<Box<dyn Token>> = scan.into_iter().collect();
        let mut index: usize = 0;
        consume_token(&v, &mut index, "map");
        consume_token(&v, &mut index, "=");
        consume_token(&v, &mut index, "{");
        consume_token(&v, &mut index, "value1");
        consume_token(&v, &mut index, "=");
        consume_token(&v, &mut index, "value2");
        consume_token(&v, &mut index, "}");
    }

    /// save map to file
    pub fn save_map(out_file: &str, map: &HashMap<String, String>) -> std::io::Result<()> {
        let f = std::fs::File::create(out_file)?;
        let mut w = std::io::BufWriter::new(f);
        writeln!(w, "map = {{")?;
        for (key, value) in map.iter() {
            writeln!(
                w,
                "\"{}\" = \"{}\"",
                convert_to_slash(key),
                convert_to_slash(value)
            )?;
        }
        writeln!(w, "}}")?;
        Ok(())
    }
    /// read map from file
    pub fn read_map(in_file: &str, map: &mut HashMap<String, String>) -> std::io::Result<()> {
        let f = std::fs::File::open(in_file)?;
        let mut s: String = String::new();
        let mut r = std::io::BufReader::new(f);
        r.read_to_string(&mut s)?;
        let scan = Scanner::new(&s);
        let v: Vec<Box<dyn Token>> = scan.into_iter().collect();
        let mut index: usize = 0;
        if v.len() > 3 {
            consume_token(&v, &mut index, "map");
            consume_token(&v, &mut index, "=");
            consume_token(&v, &mut index, "{");
            loop {
                if index < v.len() {
                    let s1 = match_token_type(&v, &mut index, TokenType::String).unwrap();
                    consume_token(&v, &mut index, "=");
                    let s2 = match_token_type(&v, &mut index, TokenType::String).unwrap();
                    map.insert(convert_from_slash(&s1), convert_from_slash(&s2));
                    if match_token(&v, index, "}") {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn save_tree_map<T>(mut writer: T, tmap: &BTreeMap<String, BTreeMap<String, String>>)
    where
        T: std::io::Write + Sized,
    {
        for (key, value) in tmap {
            if !value.is_empty() {
                writeln!(writer, "map \"{}\" = {{", key).expect("on write");
                for (key2, value2) in value {
                    writeln!(
                        writer,
                        "\"{}\" = \"{}\"",
                        convert_to_slash(key2),
                        convert_to_slash(value2)
                    )
                    .expect("on write");
                }
                writeln!(writer, "}}\n").expect("on write");
            }
        }
    }

    pub fn read_tree_map<T>(mut r: T, btmap: &mut BTreeMap<String, BTreeMap<String, String>>)
    where
        T: std::io::BufRead + Sized,
    {
        let mut s: String = String::new();
        r.read_to_string(&mut s).expect("read ");
        let scan = Scanner::new(&s);
        let v: Vec<Box<dyn Token>> = scan.into_iter().collect();
        let mut index: usize = 0;
        if v.len() > 3 {
            loop {
                consume_token(&v, &mut index, "map");
                let id = match_token_type(&v, &mut index, TokenType::String).unwrap();
                consume_token(&v, &mut index, "=");
                consume_token(&v, &mut index, "{");
                let mut tmap: BTreeMap<String, String> = BTreeMap::new();
                loop {
                    if index < v.len() {
                        let s1 = match_token_type(&v, &mut index, TokenType::String).unwrap();
                        consume_token(&v, &mut index, "=");
                        let s2 = match_token_type(&v, &mut index, TokenType::String).unwrap();
                        tmap.insert(convert_from_slash(&s1), convert_from_slash(&s2));
                        if match_token(&v, index, "}") {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                index += 1;
                btmap.insert(id, tmap);
                if index >= v.len() {
                    break;
                }
            }
        }
    }

    pub fn merge_maps(
        dst: &mut BTreeMap<String, BTreeMap<String, String>>,
        src: &BTreeMap<String, BTreeMap<String, String>>,
    ) {
        for (key, value) in src {
            dst.insert(key.to_owned(), value.to_owned());
        }
    }

    pub struct ConfigFile {
        btmap: BTreeMap<String, BTreeMap<String, String>>,
        filename: String,
    }

    impl ConfigFile {
        pub fn new(input: &str) -> Self {
            let mut map : BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
            if std::path::Path::new(input).exists() {
                let f = std::fs::File::open(input).expect("open file");
                let r = std::io::BufReader::new(f);
                read_tree_map(r, &mut map);
            }
            Self {
                btmap: map,
                filename: input.to_string()
            }
        }

        pub fn save(&mut self) {
            let f = std::fs::File::create(&self.filename).expect("on save");
            let w = std::io::BufWriter::new(f);
            save_tree_map(w, &self.btmap);
        }

        pub fn reload(&mut self) {
            let f = std::fs::File::open(&self.filename).expect("on load");
            let r = std::io::BufReader::new(f);
            read_tree_map(r, &mut self.btmap);
        }

        pub fn class_exists(&self, cls: &str) -> bool {
            self.btmap.contains_key(cls)
        }

        pub fn insert_class(&mut self, cls: &str) {
            let m : BTreeMap<String, String> = BTreeMap::new();
            self.btmap.insert(cls.to_string(), m);
        }

        pub fn set_key(&mut self, cls: &str, key: &str, value: &str) {
            if self.btmap.contains_key(cls) {
                let m = self.btmap.get_mut(cls).unwrap();
                m.insert(key.to_string(), value.to_string());
            }            
        }

        pub fn get_key(&self, cls: &str, key: &str) -> Option<String> {
            if self.btmap.contains_key(cls) {
                let m = self.btmap.get(cls).unwrap();
                return Some(m.get(key).unwrap().to_string());
            }
            None
        }
    }
}
