



fn main() -> std::io::Result<()> {
    let mut y : i32 = 10;
    let z = &mut y as *mut i32;
    unsafe {
        *z += 10;
    }
    println!("value of y: {}", y);
    Ok(())
}