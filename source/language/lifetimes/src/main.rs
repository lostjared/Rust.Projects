

fn longest<'a>(one: &'a str, two: &'a str) -> &'a str {
    if one.len() > two.len() {
        one
    } else {
        two
    }
}

fn main() {
    println!("Hello, world!");
    let longest_string = longest("one", "two three");
    println!("the longest is {}", longest_string);
}
