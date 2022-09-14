// run
// $ cargo test

#[test]
fn is_array_zero() {
    let arr = [0u8; 255];
    for i in 0..arr.len() {
        assert!(arr[i] == 0);
    }
}

#[test]
fn is_lower_case() {
    let s = "abcdefg".to_string();
    for i in s.chars() {
        assert!(i.is_ascii_lowercase());
    }
}

#[test]
fn is_index_greater_than_last() {
    let v = vec![1, 2, 3, 4];
    for i in 0..v.len()-1 {
        assert!(v[i+1] > v[i]);
    }
}


fn main() -> std::io::Result<()> {
    println!("Some tests..\n");
    Ok(())
}