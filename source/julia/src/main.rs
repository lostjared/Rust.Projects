use cmd_switch::cmd_sw;
use std::collections::HashMap;
use std::env;


pub fn parse_split_int(t: &str) -> (i32, i32) {
    let text = String::from(t.trim());
    let pos = text.find(",");
    if pos == None {
        panic!("Could not find list seperator for argument..");
    }
    let pos_value = pos.unwrap();
    let left = &text[0..pos_value];
    let right = &text[pos_value+1..text.len()];
    (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
}

pub fn parse_split_double(t: &str) -> (f32, f32) {
    let text = String::from(t.trim());
    let pos = text.find(",");
    if pos == None {
        panic!("Could not find list seperator for argument..");
    }
    let pos_value = pos.unwrap();
    let left = &text[0..pos_value];
    let right = &text[pos_value+1..text.len()];
    (left.parse::<f32>().unwrap(), right.parse::<f32>().unwrap())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut desc = HashMap::new();

    desc.insert(String::from("output"), (String::from("output file"), true));
    desc.insert(String::from("res"), (String::from("output resolution"), true));
    desc.insert(String::from("param"), (String::from("complex pair"), true));
    desc.insert(String::from("iter"), (String::from("number of iterations"), true));


    if args.len() == 1 {
        cmd_sw::print_accepted_args_map_require(&desc);
        std::process::exit(0);
    }

    let argz = cmd_sw::parse_args_require(&args, &desc);
    let output = &argz["output"];
    let res = &argz["res"];
    let res_value = parse_split_int(&res.value);
    let param = &argz["param"];
    let iter = &argz["iter"];
    let iterations = iter.value.parse::<i32>().unwrap();
    let params = parse_split_double(&param.value);

    println!("output: [{}] resolution: ({}, {}) param: ({}, {}) iterations: {}", output.value, res_value.0, res_value.1, params.0, params.1, iterations);
    draw_julia(&output.value, res_value, params, iterations);

}

pub fn draw_julia(filename: &String, res: (i32, i32), param: (f32, f32), iter: i32) {


}