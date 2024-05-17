use csv_proc::xcsv::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("incorrect arguments");
        std::process::exit(1);
    }
    let mut xcsv = XCsv::new();
    xcsv.load_file(&args[1], &',')?;
    xcsv.save_file("test1.txt", &',')?;
    Ok(())
}
