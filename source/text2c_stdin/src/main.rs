use std::io;
use std::io::prelude::*;

fn print_data() -> io::Result<()> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;
    println!("unsigned char bytes[] = {{");
    for i in &buffer {
        print!("{:#04x},", i);
    }
    println!("0x0}};");
    println!("unsigned long len = {:#04x};", buffer.len());
    Ok(())
}

fn main() -> io::Result<()> {
    print_data()?;
    Ok(())
}