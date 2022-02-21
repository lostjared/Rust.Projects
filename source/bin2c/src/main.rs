use std::env;
use std::fs::File;
use std::io::Read;

fn bin2c(infile: &str) -> std::io::Result<()> {
    let mut f = File::open(infile).expect("could not open file");
    println!("unsigned char bytes[] = {{");
    let mut len: usize = 0;
    loop {
        let mut buf: [u8; 256] = [0; 256];
        let val = f.read(&mut buf).expect("on read");

        for i in 0..val {
            print!("{:#04x},", buf[i]);
        }

        len += val as usize;

        if val == 0 {
            break;
        }
    }
    println!("0x0}};\n");
    println!("unsigned long length = {:#04x};", len);
    Ok(())
}

fn bin2c_stdin() -> std::io::Result<()> {
    println!("unsigned char bytes[] = {{");
    let mut len: usize = 0;
    loop {
        let mut buf: [u8; 256] = [0; 256];
        let val = std::io::stdin().read(&mut buf).expect("on read");
        for i in 0..val {
            print!("{:#04x},", buf[i]);
        }
        len += val as usize;

        if val == 0 {
            break;
        }
    }
    println!("0x0}};\n");
    println!("unsigned long length = {:#04x};", len);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let val1 = args.get(1).unwrap();
        bin2c(val1)?;
    } else {
        bin2c_stdin()?;
    }
    Ok(())
}
