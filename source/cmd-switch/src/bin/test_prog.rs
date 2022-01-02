use cmd_switch::cmd_sw;
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut desc = HashMap::new();
    desc.insert(
        String::from("name"),
        (String::from("enter your name"), true),
    );
    desc.insert(
        String::from("times"),
        (String::from("how many times"), true),
    );
    desc.insert(String::from("space"), (String::from("with space"), false));
    cmd_sw::print_accepted_args_map_require(&desc);
    let argz = cmd_sw::parse_args_require(&args, &desc);
    let name = &argz["name"];
    let times = &argz["times"];
    let space;
    if argz.contains_key("space") {
        space = "\t";
    } else {
        space = "";
    }
    for i in 0..times.value.parse::<i32>().unwrap() {
        println!("{}:{}{}", i, space, name.value);
    }
}
