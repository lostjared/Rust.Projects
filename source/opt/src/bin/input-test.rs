use opt::argz;
use std::io::prelude::*;

fn main() {
    let args = std::env::args().collect();
    let rt_val = argz::getopt(&args, "i:", |i: char, param: String| match i {
        'i' => {
            let f = std::fs::File::open(param).unwrap();
            let mut r = std::io::BufReader::new(f);
            let mut s: String = String::new();
            r.read_to_string(&mut s).expect("on read");
            println!("{}", s);
        }
        _ => {
            println!("argument: {}", param);
        }
    });
    if rt_val == 0 {
        println!("use input-test: -i input");
    }
}
