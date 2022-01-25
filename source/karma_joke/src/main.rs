use std::io;
fn give() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");
    String::from(input.trim())
}
fn get(value: String) -> String {
    if value.eq("love") == true {
        return String::from("1");
    }
    if value.eq("fear") == true {
        return String::from("0");
    }
    value
}
fn main() {
    loop {
        println!("Enter a thought: ");
        let thought = get(give());
        println!("karma returns: {}", thought);
    }
}
