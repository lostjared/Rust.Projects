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
    
    pub fn read_map(in_file: &str, map: &mut HashMap<String, String>) -> std::io::Result<()> {
        let f = std::fs::File::open(in_file)?;
        let mut s: String = String::new();
        let mut r = std::io::BufReader::new(f);
        r.read_to_string(&mut s)?;
        let scan = Scanner::new(&s);
        let v: Vec<Box<dyn Token>> = scan.into_iter().collect();
        let mut index = 3;
        if v.len() > 3
            && v[0].get_string() == "map"
            && v[1].get_string() == "="
            && v[2].get_string() == "{"
        {
            loop {
                if index + 1 < v.len() {
                    let s1 = v[index].get_string();
                    let s2 = v[index + 2].get_string();
                    map.insert(convert_from_slash(&s1), convert_from_slash(&s2));
                    if v[index + 3].get_string() == "}" {
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