

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
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        let input = std::io::stdin();
        let l = input.lock();
        output_hex(l);
    } else {
        for i in args.iter().skip(1) {
            let f = std::fs::File::open(i).unwrap();
            let r = std::io::BufReader::new(f);
            output_hex(r);
        }
    }
}
