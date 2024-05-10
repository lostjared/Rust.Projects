
struct Type {
    name: String,
    id: u32,
}

impl Type {
    fn new() -> Self {
        Self {
            name: "Jared".to_string(),
            id: 250
        }
    }
    fn set_id(&mut self, i: u32) {
        self.id = i;
    }
}

impl Drop for Type {
    fn drop(&mut self)  {
        println!("Dropping {}:{}", self.name, self.id);
        // clean up here same as destructor in C++
    }
}

fn main() -> std::io::Result<()> {
    let mut t = Type::new();
    t.set_id(100);
    Ok(())
}
