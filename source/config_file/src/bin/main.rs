extern crate config_file;

use config_file::config::Config;
use std::collections::HashMap;
fn main() {
    let map : HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut config = Config::create(map, "test.cfg".to_string());
    //config.setkey(&"id".to_string(), &"one".to_string(), &"value".to_string());
    //config.setkey(&"id".to_string(), &"two".to_string(), &"value2".to_string());
    //config.save();
    config.load();
    let value = config.getkey(&"id".to_string(), &"one".to_string());
    match value {
        Some(v) => {
            println!("The value is: {}", v);
        }
        None => {
            println!("value not found!\n");
        }
    }
}