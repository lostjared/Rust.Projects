use opt::argz;

fn main() {
    let args = std::env::args().collect();
    argz::getopt(&args, "i:o:a", |i: char, param: String| match i {
        'i' => {
            println!("Input argument: {}", param);
        }
        'o' => {
            println!("Output argument: {}", param);
        }
        'a' => {
            println!("A argument\n");
        }
        _ => {
            println!("unknown argument ");
        }
    });
}