use logger::log::*;

fn main() -> std::io::Result<()> {
    let mut log = Log::new_log_file("log.txt");
    log.i(&format!("Logging to file\n"));
    log.e(&format!("logging an error\n"));
    log.o(&format!("standard log\n"));
    Ok(())
}
