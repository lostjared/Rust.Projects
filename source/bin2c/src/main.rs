
use std::fs::File;
use std::io::Read;
use std::env;

fn bin2c(infile: &str) -> std::io::Result<()> {
    let mut f = File::open(infile).expect("could not open file");
    println!("unsigned char bytes[] = {{");
    loop {
        let mut buf : [u8; 256] = [0; 256];
        let val = f.read(&mut buf).expect("on read");

        for i in 0..val {
            print!("{:#04x},", buf[i]);
        }

        if val == 0 {
            break;
        }
    }
    println!("0x0}};\n");
    Ok(())
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let val1 = args.get(1).unwrap();
        bin2c(val1)?;
    }
    else {
        println!("Error: input_file");
    }
    Ok(())
}
