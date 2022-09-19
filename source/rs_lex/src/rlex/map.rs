pub mod rs_map {

    use crate::rlex::{convert_from_slash, convert_to_slash, Scanner, Token};
    use std::collections::HashMap;
    use std::io::Write;
    use std::io::Read;

    pub fn save_map(out_file: &str, map: &HashMap<String, String>) -> std::io::Result<()> {
        let f = std::fs::File::create(out_file)?;
        let mut w = std::io::BufWriter::new(f);
        writeln!(w, "map = {{")?;
        for (key, value) in map.iter() {
             writeln!(w,"\"{}\" = \"{}\"",
            convert_to_slash(key),
            convert_to_slash(value))?;
        }
        writeln!(w,"}}")?;
        Ok(())
    }

    pub fn consume_token(v: &Vec<Box<dyn Token>>, index: &mut usize, tok: &str) {
        if v[*index].get_string() == tok.to_string() {
            *index += 1;
        } else {
            panic!("Expected: {} found {}", tok, v[*index].get_string());
        }
    }

    pub fn match_token(v: &Vec<Box<dyn Token>>, index: usize, tok: &str) -> bool {
        if v[index].get_string() == tok.to_string() {
            return true;
        } else {
            return false;
        }
    }
    
    pub fn read_map(in_file: &str, map: &mut HashMap<String, String>) -> std::io::Result<()> {
        let f = std::fs::File::open(in_file)?;
        let mut s: String = String::new();
        let mut r = std::io::BufReader::new(f);
        r.read_to_string(&mut s)?;
        let scan = Scanner::new(&s);
        let v: Vec<Box<dyn Token>> = scan.into_iter().collect();
        let mut index : usize = 0;
        if v.len() > 3 {
            consume_token(&v,&mut index, "map");
            consume_token(&v,&mut index, "=");
            consume_token(&v,&mut index, "{");
            loop {
                if index + 3 < v.len() {
                    let s1 = v[index].get_string();
                    let s2 = v[index + 2].get_string();
                    map.insert(convert_from_slash(&s1), convert_from_slash(&s2));
                    if match_token(&v, index + 3, "}") {
                        break;
                    }
                    index += 3;
                } else {
                    break;
                }
            }
        }
    
        Ok(())
    }
}