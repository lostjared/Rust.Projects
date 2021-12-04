
struct PairType<T> {
    one: T,
    two: T
}

impl<T: Copy> PairType<T> {

    fn set(&mut self, o: &T, t: &T) {
        self.one = *o;
        self.two = *t;
    }

    fn get_one<'a>(&'a self) -> &'a T {
        &self.one
    }

    fn get_two<'a>(&'a self) -> &'a T {
        &self.two
    }
}

fn main() {
    let mut t : PairType<u32> = PairType{ one: 1, two: 2 };
    println!("{}:{}", t.one, t.two);
    t.set(&0, &0);
    println!("{}:{}", t.one, t.two);
    println!("{}:{}", *t.get_one(), *t.get_two());
}
