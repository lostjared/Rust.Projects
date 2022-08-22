fn convert_to_rs<T>(mut reader: T) -> String
where
    T: std::io::BufRead + Sized,
{
    let mut value: String = String::new();
    value.push_str("let v : Vec<u8> = vec![");
    let mut bytes = [0; 1024];
    let mut first_byte = true;

    loop {
        let val = reader.read(&mut bytes).expect("on read");
        if val == 0 {
            break;
        } else {
            if first_byte == true {
                first_byte = false;
            } else {
                value.push(',');
            }
        }
        for i in 0..val-1 {
            let s = format!("{:#04x},", bytes[i]);
            value.push_str(&s);
        }
        let fc = format!("{:#04x}", bytes[val-1]);
        value.push_str(&fc);
    }
    value.push_str("];\n");
    value
}

fn echo_vec(output : &Vec<u8>) {
    for i in output {
        print!("{}", *i as char);
    }
}

fn main() {
    let v : Vec<u8> = vec![0x54,0x68,0x69,0x73,0x20,0x70,0x72,0x6f,0x67,0x72,0x61,0x6d,0x20,0x77,0x69,0x6c,0x6c,0x20,0x74,0x75,0x72,0x6e,0x20,0x61,0x20,0x73,0x74,0x72,0x69,0x6e,0x67,0x20,0x69,0x6e,0x74,0x6f,0x20,0x61,0x20,0x76,0x65,0x63,0x74,0x6f,0x72,0x20,0x6f,0x66,0x20,0x62,0x79,0x74,0x65,0x73,0x20,0x69,0x6e,0x20,0x52,0x75,0x73,0x74,0x0a];
    echo_vec(&v);
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
