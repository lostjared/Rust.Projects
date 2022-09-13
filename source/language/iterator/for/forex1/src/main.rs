


fn main() -> std::io::Result<()> {
    let v = vec![2, 4, 3, 2, 1, 9];
    for i in &v {
        println!("{} i", i);
    }
    for (num, i) in v.iter().enumerate() {
        println!("{}: {}", num, i);

    }
    v.into_iter().for_each(|i| { println!("for each: {}", i); });
    Ok(())
}