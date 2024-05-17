use csv_proc::xcsv::*;

#[test]
fn test_csv() {
    let mut xcsv = XCsv::new();
    xcsv.add_row(&["test", "one", "two", "three"]);
    xcsv.add_row(&["apples", "bannas", "rocks","dirt"]);
    assert_eq!(xcsv.at(0, 1), "one");
    assert_eq!(xcsv.at(1, 1), "bannas");
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("incorrect arguments");
        std::process::exit(1);
    }
    let mut xcsv = XCsv::new();
    xcsv.load_file(&args[1], &',')?;
    xcsv.add_row(&["Apple", "Data", "Siren", "Thought"]);
    xcsv.save_file("test1.txt", &',')?;
    println!("{}", xcsv.at(0, 1));
    println!("{}", xcsv.at(3, 2));

    Ok(())
}
