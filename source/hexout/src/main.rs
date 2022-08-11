/*
fn output_hex<T: std::io::BufRead + Sized>(mut reader: T) {
    let mut counter = 0;
    loop {
        let mut buf: [u8; 256] = [0; 256];
        let val = reader.read(&mut buf).expect("on read");
        for i in 0..val {
            print!("{:#04x} ", buf[i]);
            counter += 1;
            if counter % 6 == 0 {
                print!("\n");
            }
        }
        if val == 0 {
            break;
        }
    }
}*/

fn output_hex_line<T: std::io::BufRead + Sized>(mut reader: T) {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).expect("on read");
    let mut index = 0;
    while index < buffer.len() {
        for i in 0..6 {
            if i + index < buffer.len() {
                print!("{:#04x} ", buffer[i + index]);
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
                    print!("{}", buffer[i + index] as char);
                }
            } else {
                print!(".");
            }
        }
        index += 6;
        println!("\n");
    }
}

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
