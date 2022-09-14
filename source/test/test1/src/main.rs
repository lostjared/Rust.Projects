
// run 
// $ cargo test

#[test]
fn is_vector_sorted() {
    let mut v = vec![2, 1, 4, 5];
    v.sort();
    for i in 0..v.len()-1 {
        assert_eq!(v[i+1] >= v[i] , true);
    }
    v.sort_by(|a, b| b.cmp(a));
    for i in 0..v.len()-1 {
        assert_eq!(v[i+1] <= v[i], true)
    }
}

fn main() -> std::io::Result<()> {
    let mut v = vec![2, 1, 4, 5];
    v.sort();
    v.into_iter().for_each(|i| println!("{}", i));
    Ok(())
}