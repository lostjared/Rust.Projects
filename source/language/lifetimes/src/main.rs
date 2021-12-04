

fn longest<'a>(one: &'a str, two: &'a str) -> &'a str {
    if one.len() > two.len() {
        one
    } else {
        two
    }
}

#[test]
fn test_longest() {
    assert_eq!(longest("o", "oo"), "oo");
}

fn main() {
    println!("Hello, world!");
    let longest_string = longest("one", "two three");
    println!("the longest is {}", longest_string);
}
