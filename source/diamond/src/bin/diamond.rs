use diamond::dmd;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let d = dmd::Diamond::new(&args);
    for i in d {
        println!("{}", i);
    }
}
