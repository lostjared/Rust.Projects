use rs_lex::rlex::map::rs_map::*;
use std::collections::BTreeMap;

fn run_count() -> usize {
    let mut btmap : BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    let mut count = 0;
    if std::path::Path::new("count.dat").exists() {
        let f = std::fs::File::open("count.dat").expect("on open");
        let r = std::io::BufReader::new(f);
        read_tree_map(r, &mut btmap);    
        if btmap.contains_key("program") {
            let m = btmap.get("program").unwrap();
            let c = m.get("count").unwrap();
            count = c.parse().unwrap();
        } else {
            count = 0;
        }
    }
    count += 1;
    let mut program : BTreeMap<String, String> = BTreeMap::new();
    program.insert("count".to_string(), count.to_string());
    btmap.insert("program".to_string(), program);
    let f = std::fs::File::create("count.dat").expect("on create");
    let w = std::io::BufWriter::new(f);
    save_tree_map(w, &btmap);
    count
}

fn main() -> std::io::Result<()> {
    println!("Program ran: {} times", run_count());
    Ok(())
}