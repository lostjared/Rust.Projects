extern crate config_file;

use config_file::config::Config;
use std::collections::HashMap;
fn main() {
    let map : HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut config = Config::create(map, "test.cfg");
 //   config.setkey("id", "one", "value");
 //   config.setkey("id","two","value2");
 //   config.save();
    config.load();
    let value = config.getkey("id", "one");
    match value {
        Some(v) => {
            println!("The value is: {}", v);
        }
        None => {
            println!("value not found!\n");
        }
    }
}