use std::env;
use std::fs;

fn rename_files(file_list: &String, name_prefix: &String) {
    let mut file_counter : u64 = 0;
    let contents = fs::read_to_string(file_list).expect("Error reading the file");
    let value = contents.lines();
    for i in value {
        if i.len() > 0 {
            let mut prefix_string = String::new();
            prefix_string.push_str(&format!("{}{}_{}",name_prefix,file_counter+1, i));
            println!("{} => {}", i, prefix_string);
            fs::rename(i, prefix_string).expect("error on copy");
            file_counter += 1;
        }
    }
    println!("copied {} file(s)", file_counter);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let val1 = args.get(1).unwrap();
        let val2 = args.get(2).unwrap();
        rename_files(val1, val2);
    } else {
        println!("Error: input_file prefix");
    }
}
