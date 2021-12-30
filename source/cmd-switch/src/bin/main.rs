use std::collections::HashMap;
use cmd_switch::cmd_sw;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut desc = HashMap::new();
    desc.insert(String::from("hello"), String::from("enter your name"));
    cmd_sw::print_accepted_args(&desc);
    let argz = cmd_sw::parse_args(&args, &desc);
    for (key, value) in &argz {
        println!("{} = {}", *key, value.value);
    }
    if !argz.contains_key("hello") {
        println!("Requires --hello=name ");
        std::process::exit(0);
    }
    let v = argz.get("hello").unwrap();
    println!("Key is {} value is {} its for {}", v.key, v.value, v.desc);

}