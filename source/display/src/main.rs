
use std::fmt::{self, Formatter, Display};

struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "x: {} y: {} z: {}", self.x, self.y, self.z)
    }
}

fn main() {
    let p : Point = Point {x: 10, y: 20, z: 30 };
    println!("Point: {}", p);
}
