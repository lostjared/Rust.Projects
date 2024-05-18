
use csv_proc::xcsv::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("incorrect arguments");
        std::process::exit(1);
    }
    let mut xcsv = XCsv::new();
    xcsv.load_file(&args[1], &',')?;
    let mut index = 0;
    for row in xcsv.table {
        println!("Row [{}]", index);
        let mut col_index = 0;
        for col in row {
            println!("\t Col[{}] ->\t {}", col_index, col);
            col_index += 1;
        }
        index += 1;
    }
    Ok(())
}