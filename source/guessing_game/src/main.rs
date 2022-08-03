use rand::Rng;
use std::io;

fn guess_number() -> u32 {
    println!("Enter number: ");
    let mut input_text: String = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("error on readline");
    input_text.pop();
    input_text.parse().unwrap()
}

fn main() {
    loop {
        println!("Guessing game guess between 1-10");
        let value = guess_number();
        let mut random = rand::thread_rng();
        if value == random.gen_range(1..10) {
            println!("Correct you win...\n");
        } else {
            println!("Incorrect you lose..\n");
        }
        println!("Try again (yes/no)");
        let mut input_text: String = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("error on readline");
        input_text.pop();
        if input_text != "yes" {
            break;
        }
    }
}
