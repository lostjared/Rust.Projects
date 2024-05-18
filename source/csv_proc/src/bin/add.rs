use csv_proc::xcsv::*;
use std::io::BufRead;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("incorrect arguments");
        eprintln!("use: input_file output_file");
        std::process::exit(1);
    }
    let mut xcsv = XCsv::new();
    if args[1] != "-n" {
        xcsv.load_file(&args[1], &',')?;
    }
    println!("Enter lines one at a time when completed type break");
    loop {
        print!("> ");
        std::io::stdout().lock().flush()?;
        let mut line = String::new();
        let mut lock = std::io::stdin().lock();
        lock.read_line(&mut line)?;
        let line_t = line.trim_end();
        if line_t == "break" {
            break;
        }
        let mut temp_xcsv = XCsv::new();
        temp_xcsv.load_string(&line, &',');
        let len = temp_xcsv.table.len();
        for row in temp_xcsv.table {
            println!("Wrote {} col(s) on row", row.len());
            xcsv.table.push(row);
        }
        println!("Wrote {} rows(s) to table", len);
    }
    xcsv.save_file(&args[2], &',')?;
    println!("Wrote updated file to: {}", args[2]);
    Ok(())

}
