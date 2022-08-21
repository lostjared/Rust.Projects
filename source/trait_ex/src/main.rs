// polymorphism with traits

struct Monkey {
    name: String,
}
struct Cat {
    name: String,
}
struct Human {
    name: String,
}

trait Animal {
    fn eat(&self);
    fn die(&self);
}

impl Animal for Monkey {
    fn eat(&self) {
        println!("Monkey named: {} eats something", self.name);
    }

    fn die(&self) {
        println!("Monkey dies");
    }
}

impl Animal for Cat {
    fn eat(&self) {
        println!("Cat named {} eats something", self.name);
    }

    fn die(&self) {
        println!("Cat dies");
    }
}

impl Animal for Human {
    fn eat(&self) {
        println!("Human named {} eats something", self.name);
    }

    fn die(&self) {
        println!("Human dies");
    }
}

fn eat_and_die(o: &dyn Animal) {
    o.eat();
    o.die();
}

fn generic_eat<T>(animal: &T)
where
    T: Animal,
{
    animal.eat();
}

fn main() {
    let human = Human {
        name: "Jared".to_string(),
    };
    let monkey = Monkey {
        name: "Bobo".to_string(),
    };
    let cat = Cat {
        name: "Coder".to_string(),
    };
    eat_and_die(&human);
    eat_and_die(&monkey);
    eat_and_die(&cat);

    let vec: Vec<&dyn Animal> = vec![&human, &monkey, &cat];
    for i in vec {
        eat_and_die(i);
    }
    generic_eat(&human);
}
