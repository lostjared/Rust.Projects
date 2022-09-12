

fn main() -> std::io::Result<()> {
    let v = vec![2, 4, 6, 8, 10];
    let i : Vec<i32>  = v.into_iter().map(|x| x * x).collect();
    println!("{:?}", i);
    Ok(())
}