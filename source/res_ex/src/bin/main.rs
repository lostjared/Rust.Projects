
use res_ex::res_ex::extract_resolution;

fn main() -> std::io::Result<()> {

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        let a = extract_resolution(&args[1]);
        println!("Width: {}, Height: {}", a.width, a.height);
    } else {
        panic!("Requires one argument... wxh");
    }

    Ok(())
}