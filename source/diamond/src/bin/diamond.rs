use diamond::dmd;
/// main function example
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let d = dmd::Diamond::new(&args);
    println!("Diamond: {}", d);
    for i in d {
        println!("{}", i);
    }
}
