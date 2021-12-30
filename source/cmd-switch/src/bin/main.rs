
use cmd_switch::cmd_sw;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argz = cmd_sw::parse_args(&args);
    for (key, value) in &argz {
        println!("{} = {}", *key, value.value);
    }
    if !argz.contains_key("hello") {
        println!("Requires --hello=name ");
        std::process::exit(0);
    }
    let v = argz.get("hello").unwrap();
    println!("Key is {} value is {}", v.key, v.value);
}