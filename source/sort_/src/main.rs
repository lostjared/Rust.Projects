use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let mut v : Vec<String> = Vec::new();
    for i in std::io::stdin().lock().lines() {
        let i = i.unwrap();
        if !i.is_empty() {
            v.push(i);
        }
    }
    v.sort();
    for i in &v {
        println!("{}", i);
    }
    Ok(())
}