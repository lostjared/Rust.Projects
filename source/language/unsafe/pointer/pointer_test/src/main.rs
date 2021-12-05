

fn set_val(p: *mut u32) {
    unsafe {
        *p = 25;
    }
}


fn main() {
    let mut x : u32 = 10;
    let y : *mut u32 = &mut x;
    unsafe {
        *y = 20;
        println!("x: {} y: {}", x, *y);
        set_val(y);
        println!("y: {}", *y);
    }
}
