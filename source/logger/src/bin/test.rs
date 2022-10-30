use logger::log::*;
use std::io::BufRead;

fn log_output(log: &mut Log) {
    log.i(format!("Logging to file"));
    log.e(format!("logging an error"));
    log.o(format!("standard log"));
    write!(log.fd(), "Wrote directly so stream\n").expect("on write");
    write!(log.fd(), "Wrote at: {}\n", the_time()).expect("on write");
}


fn main() -> std::io::Result<()> {
    let mut s = String::new();
    println!("Write to: enter 1 for file, 2 for stdout, 3 for fatal");
    std::io::stdin().lock().read_line(&mut s).expect("on read");
    let sv = s.trim();
    if sv == "1" {
        log_output(&mut Log::new_log_file("log.txt"));
    } else if sv == "2" {
        log_output(&mut Log::new_stdout_log());
    } else if sv == "3" {
        let mut log = Log::new_stdout_log();
        log.i(format!("program running...."));
        log.f(format!("Fatal Error\n"));
    } else {
        log_output(&mut Log::new_stderr_log());
    }
    Ok(())
}
