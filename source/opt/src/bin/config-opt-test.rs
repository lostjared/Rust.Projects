use opt::argz;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut t_op = false;
    let mut o_op = false;
    let mut input: String = String::new();

    let count = argz::getopt(&args, "toi:", |i: char, param: String| match i {
        't' => {
            t_op = true;
        }
        'o' => {
            o_op = true;
        }
        'i' => {
            input = param;
        }
        _ => {
            println!("arg: {}", param);
        }
    });
    println!(
        "Argument count: {} values: t: {} o: {} i: {}",
        count, t_op, o_op, input
    );
}
