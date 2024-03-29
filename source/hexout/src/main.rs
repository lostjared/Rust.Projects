
/// output hex data to stdout
fn output_hex_line<T: std::io::BufRead + Sized>(mut reader: T) {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).expect("on read");
    let mut index = 0;
    while index < buffer.len() {
        for i in 0..6 {
            if i + index < buffer.len() {
                print!("{:#04x} ", buffer[i + index]);
            } else {
                print!("{:04x} ", 0);
            }
        }
        for i in 0..6 {
            if i + index < buffer.len() {
                if buffer[i + index] == 32
                    || buffer[i + index] == 10
                    || buffer[i + index] == 13
                    || buffer[i + index] == 9
                {
                    print!(".");
                } else {
                    let ch = buffer[i + index] as char;
                    if ch.is_ascii_alphanumeric() || ch.is_ascii_graphic() {
                        print!("{}", buffer[i + index] as char);
                    } else {
                        print!(".");
                    }
                }
            } else {
                print!(".");
            }
        }
        index += 6;
        println!("\n");
    }
}

/// main function - entry point
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        let input = std::io::stdin();
        let l = input.lock();
        output_hex_line(l);
    } else {
        for i in args.iter().skip(1) {
            let f = std::fs::File::open(i).unwrap();
            let r = std::io::BufReader::new(f);
            output_hex_line(r);
        }
    }
}
