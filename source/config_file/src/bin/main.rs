extern crate config_file;

use config_file::config::Config;

fn main() -> std::io::Result<()> {
    let mut config = Config::create("test.cfg");
    //   config.setkey("id", "one", "value");
    //   config.setkey("id","two","value2");
    //   config.save();
    config.load()?;
    let value = config.getkey("program", "ran");
    match value {
        Some(v) => {
            println!("The value is: {}", v);
            let run = v.parse::<i32>();
            match run {
                Ok(r) => {
                    let ran = r + 1;
                    config.setkey("program", "ran", &format!("{}", ran));
                    println!("Program ran: {} ", ran);
                    config.save();
                    println!("config values: {}", config);
                }
                Err(e) => {
                    println!("Error converting data: {}...\n", e);
                }
            }
        }
        None => {
            println!("Program ran: 1");
            config.setkey("program", "ran", "1");
            config.save();
        }
    }
    Ok(())
}
