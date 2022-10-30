use logger::log::*;
use std::io::BufRead;

fn log_output(log: &mut Log) {
    log.i(format!("Info Logging"));
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
    let mut log = match sv {
        "1" => Log::new_log_file("Example", "log.txt"), 
        "2" => Log::new_stdout_log("Example"),
        "3" => Log::new_stdout_log("Example"),
        _=> Log::new_stderr_log("Example")
    };
    log.i(format!("Program running"));
    log_output(&mut log);
    if sv == "3" {
        log.f(format!("Fatal"));
    }
    Ok(())
}
