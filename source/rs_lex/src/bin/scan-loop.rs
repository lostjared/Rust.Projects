use rs_lex::rlex::*;
use std::io::Write;

/// proper way to do the loop
fn scan_text<T>(reader: T)
where
    T: std::io::BufRead + Sized,
{
    print!("> ");
    std::io::stdout().lock().flush().expect("on flush");
    for line in reader.lines() {
        let mut rlex = rs_lex::rlex::Scanner::new(&line.unwrap());
        loop {
            let token = rlex.scan_token();
            match token {
                ScanResult::Error => {
                    eprintln!("Scanner Error ");
                    break;
                }
                ScanResult::Ok(tok) => match tok {
                    Some(i) => {
                        let id = format!("{:?}", i.get_type());
                        println!("{:15} -> {}", id, i.get_string());
                    }
                    None => {
                        break;
                    }
                },
            }
        }
        print!("> ");
        std::io::stdout().lock().flush().expect("on flush");
    }
}
/// main function
pub fn main() -> std::io::Result<()> {
    scan_text(std::io::stdin().lock());
    Ok(())
}
