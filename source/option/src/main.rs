
fn value(i: i32) -> Option<i32> {
    if i == 0 {
        return Some(10);
    } else if i == 1 {
        return Some(20)
    }
    None
}

#[test]
fn value_test() {
    assert_eq!(value(0), Some(10));
    assert_eq!(value(1), Some(20));
    assert_eq!(value(2), None);
}


fn main() {
    println!("option test");
}
