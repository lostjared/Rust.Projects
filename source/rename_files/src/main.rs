use std::env;
use std::fs;

fn rename_files(file_list: &String, name_prefix: &String) {
    let mut file_counter : u64 = 0;
    let contents = fs::read_to_string(file_list).expect("Error reading the file");
    let counter = contents.matches("\n").count();
    let sval = format!("{}", counter);
    for i in contents.lines() {
        if i.len() > 0 {
            let prefix_string = String::from(&format!("{}{:0width$}_{}",name_prefix,file_counter+1, i, width=sval.len()));
            println!("{} => {}", i, prefix_string);
            fs::rename(i, prefix_string).expect("error on rename");
            file_counter += 1;
        }
    }
    println!("renamed {} file(s)", file_counter);
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
