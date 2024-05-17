use csv_proc::xcsv::*;

#[test]
fn test_csv() {
    let mut xcsv = XCsv::new();
    xcsv.add_row(&["test", "one", "two", "three"]);
    xcsv.add_row(&["apples", "bannas", "rocks","dirt"]);
    assert_eq!(xcsv.at(0, 1), "one");
    assert_eq!(xcsv.at(1, 1), "bannas");
    for row in &xcsv.table {
        for item in row {
            assert!(item.len() > 0);
        }
    }
}

#[test]
fn test_load_string() {
    let mut xcsv = XCsv::new();
    xcsv.load_string("apples,oranges,grapefruit,chocolate\nhere,there,overhre,nowhere\n1,2,3,4\n\"Here is a string\",one,two,three", &',');
    assert_eq!(xcsv.at(0, 1), "oranges");
    assert_eq!(xcsv.at(2, 2), "3");
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
