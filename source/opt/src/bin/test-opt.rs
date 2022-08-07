use opt::argz;

fn main() {
    let args = std::env::args().collect();
    let count = argz::getopt(&args, "i:o:a", |i: char, param: String| match i {
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

    if count == 0 {
        println!("Program use:\n-i argument\n-o argument\n-a");
    }
}
