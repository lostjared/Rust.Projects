// polymorphism with traits

struct Monkey {}
struct Cat {}
struct Human {}

trait Animal {
    fn eat(&self);
    fn die(&self);
}

impl Animal for Monkey {
    fn eat(&self) {
        println!("Monkey eats something");
    }

    fn die(&self) {
        println!("Monkey dies");
    }
}

impl Animal for Cat {
    fn eat(&self) {
        println!("Cat eats something");
    }

    fn die(&self) {
        println!("Cat dies");
    }
}

impl Animal for Human {
    fn eat(&self) {
        println!("Human eats something");
    }

    fn die(&self) {
        println!("Human dies");
    }
}

fn eat_and_die(o: &dyn Animal) {
    o.eat();
    o.die();
}

fn main() {
    let human = Human {};
    let monkey = Monkey {};
    let cat = Cat {};
    eat_and_die(&human);
    eat_and_die(&monkey);
    eat_and_die(&cat);
    
    let vec : Vec<&dyn Animal> = vec![&human, &monkey, &cat];
    for i in vec {
        eat_and_die(i);
    }
}
