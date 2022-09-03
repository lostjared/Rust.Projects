
fn echo_all() {
    let args : Vec<String> = std::env::args().collect();
    for i in args.iter().skip(1) {
        print!("{} ", i);
    }
    println!("");
}

fn main() -> std::io::Result<()> {
    echo_all();
    Ok(())
}
