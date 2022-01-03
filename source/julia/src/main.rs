use cmd_switch::cmd_sw;
use std::collections::HashMap;
use std::env;


pub fn parse_split_int(text: &str) -> (i32, i32) {
    let pos = text.find(",");
    if pos == None {
        panic!("Could not find list seperator for argument..");
    }
    let pos_value = pos.unwrap();
    let left = &text[0..pos_value];
    let right = &text[pos_value+1..text.len()];
    (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut desc = HashMap::new();

    desc.insert(String::from("output"), (String::from("output file"), true));
    desc.insert(String::from("res"), (String::from("output resolution"), true));

    if args.len() == 1 {
        cmd_sw::print_accepted_args_map_require(&desc);
        std::process::exit(0);
    }

    let argz = cmd_sw::parse_args_require(&args, &desc);

    let output = &argz["output"];
    let res = &argz["res"];
    let res_value = parse_split_int(&res.value);

    println!("output: [{}] resolution: ({}, {})", output.value, res_value.0, res_value.1);

}
