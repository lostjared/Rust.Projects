

fn set_value(y: *mut i32) {
    unsafe {
        *y += 10;
    }
}


fn main() -> std::io::Result<()> {
    let mut y : i32 = 10;
    let z = &mut y as *mut i32;
    unsafe {
        *z += 10;
    }
    println!("value of y: {}", y);
    set_value(z);
    println!("value of y: {}", y);
    let q : i32 = 10;
    let ptr_q =  &q as *const i32;
    unsafe {
        println!("value of ptr_q: {}", *ptr_q);
    }

    Ok(())
}