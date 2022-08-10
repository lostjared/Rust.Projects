use diamond::dmd;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let d = dmd::Diamond::new(&args);
    /*
    loop {
        let line = d.read_next();
        if line == None {
            break;
        }
        println!("{}", line.unwrap());
    } */
    // or
    for i in d {
        println!("{}", i);
    }
}
