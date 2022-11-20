use std::io::Read;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        let val = &args[1];
        let sep = val.find(',');
        if sep == None {
            panic!("Missing required argument , ");
        }
        let mut string_value: String = String::new();
        let mut reader = std::io::stdin().lock();
        reader.read_to_string(&mut string_value).expect("on read");
        let sep = sep.unwrap();
        let num1 = &val[0..sep];
        let num2 = &val[sep + 1..];
        let start_pos: usize;
        let stop_pos: usize;
        if num1 == "$" {
            start_pos = 0;
        } else {
            start_pos = num1.parse().unwrap();
        }
        if num2 == "$" {
            stop_pos = string_value.len();
        } else {
            stop_pos = num2.parse().unwrap();
        }
        if start_pos < string_value.len() && stop_pos > start_pos {
            let cut_value = &string_value[start_pos..stop_pos];
            println!("{}", cut_value);
        }
    }
    Ok(())
}
