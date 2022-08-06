
use opt::argz;

fn main() {
    let args = std::env::args().collect();
    argz::optarg(&args,"t:x", |i: char, param: String| {
        if param.len() > 0 {
            println!("ch: {} param: {} ", i, param);
        } else {
            println!("chars: {}", i);
        }
    });

}