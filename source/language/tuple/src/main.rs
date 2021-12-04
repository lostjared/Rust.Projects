
fn values(one: u32) -> (u32, u32, f32) {
    if one == 1 {
        (10, 10, 10.5)
    } else {
        (5, 5, 5.5)
    }
}

fn print_values(x : (u32, u32, u32, u32)) {
    println!("{}: {}: {}: {}", x.0, x.1, x.2, x.3);
}

#[test] 
fn tuple_test() {
    assert_eq!(values(1), (10, 10, 10.5));
    assert_eq!(values(0), (5, 5, 5.5));
}

fn main() {
    let val = values(1);
    let val2 = values(2);
    println!("{}: {}: {}", val.0, val.1, val.2);
    println!("{}: {}: {}", val2.0, val2.1, val2.2);
    let val3 = (20, 20, 20.5);
    println!("{}: {}: {}", val3.0, val3.1, val3.2);
    print_values((0, 1, 2, 3));
}

