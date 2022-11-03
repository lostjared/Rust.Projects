use logger::log::*;
use std::io::BufRead;

fn log_output(log: &mut Log) {
    log.i("Info Logging");
    log.e("logging an error");
    log.o("standard log");
    writeln!(log.fd(), "Wrote directly so stream").expect("on write");
    writeln!(log.fd(), "Wrote at: {}", the_time()).expect("on write");
}

fn main() -> std::io::Result<()> {
    let mut s = String::new();
    println!("Write to: enter 1 for file, 2 for stdout, 3 for fatal");
    std::io::stdin().lock().read_line(&mut s).expect("on read");
    let sv = s.trim();
    let mut log = match sv {
        "1" => Log::new_file_log("Example", "log.txt", true, true),
        "2" => Log::new_stdout_log("Example"),
        "3" => Log::new_stdout_log("Example"),
        _ => Log::new_stderr_log("Example"),
    };
    log.i(&format!("Program running: time {}", the_time()));
    log_output(&mut log);
    if sv == "3" {
        log.f("Fatal");
    }
    Ok(())
}
