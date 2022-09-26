use rs_lex::rlex::map::rs_map::*;

fn run_count() -> usize {
    let mut count = 0;
    let mut config = ConfigFile::new("count.dat");
    if config.class_exists("program") {
        count = config.get_key("program", "count").unwrap().parse().unwrap();
    } else {
        config.insert_class("program");
    }
    count += 1;
    config.set_key("program", "count", &count.to_string());
    config.save();
    count
}

fn main() -> std::io::Result<()> {
    println!("Program ran: {} times", run_count());
    Ok(())
}
