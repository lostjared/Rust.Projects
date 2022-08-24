
use std::ops::{Add, Mul};

#[derive(Copy,Clone,Debug)]
struct IntegerPair {
    x: i64,
    y: i64
}

impl Add for IntegerPair {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Mul for IntegerPair {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

fn main() -> std::io::Result<()> {
    let x = IntegerPair{ x: 10, y: 10 };
    let y = IntegerPair{ x: 20, y: 20 };
    let value = x * x + y;
    println!("{:?}*{:?}+{:?} = {:?}", x, x, y, value);
    Ok(())
}