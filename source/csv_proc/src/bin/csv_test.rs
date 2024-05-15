// Like a CSV file but with syntax of a programming language
// called a csv_proc
// each row/col contains one Token either an Idenetifier, String, Digits, Symbol, or Single String
// to allow more than one type surround in quotes

use csv_proc::csv::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Error requires argument file to proc");
        std::process::exit(1);
    }
    let mut csv = Csv::new();
    csv.load_file(&args[1], ",")?;
    for row in &csv.data {
        let size = row.len();
        if size > 0 {
            print!("ROW [");
            for i in &*row {
                print!(" {} ", i);
            }
            println!("]");
        }
    }
    println!(" row 0 col 1 {}", csv.data[0][1]);
    Ok(())
}
