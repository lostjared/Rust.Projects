fn convert_to_rs<T: std::io::BufRead + Sized>(mut reader: T) -> String {
    let mut value: String = String::new();
    value.push_str("let v = vec![");
    let mut bytes = [0; 1024];
    loop {
        let val = reader
            .read(&mut bytes)
            .expect("on read");
            if val == 0 {
                break;
            }
            for i in 0..val {
                let s = format!("{:#04x},", bytes[i]);
                value.push_str(&s);
            }
        }
    value.push_str("0x0];\n");
    value
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        let i = std::io::stdin();
        let r = i.lock();
        let s: String = convert_to_rs(r);
        println!("{}", s);
    } else {
        for i in args.iter().skip(1) {
            let f = std::fs::File::open(i).unwrap();
            let r = std::io::BufReader::new(f);
            let s: String = convert_to_rs(r);
            println!("{}", s);
        }
    }
}