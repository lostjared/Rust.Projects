
pub struct Object {
    index: std::sync::Mutex<i64>,
}

impl Object {

    pub fn new(i: i64) -> Object {
        let ind = std::sync::Mutex::<i64>::new(i); 
        Object {
            index: ind,
        }
    }

    pub fn inc(&self) {
        *self.index.lock().unwrap() += 1;
    }

    pub fn dec(&self) {
        *self.index.lock().unwrap() -= 1;
    }

    pub fn value(&self) -> i64 {
        *self.index.lock().unwrap()        
    }

    pub fn echo(&self) {
        println!("{}", *self.index.lock().unwrap());
    }
}

fn main() {
    let obj = std::sync::Arc::new(Object::new(100));
    let obj2 = obj.clone();
    let th1 = std::thread::spawn(move || {
        obj2.echo();
        loop {
            println!("THREAD 1 -> ");
            obj2.inc();
            obj2.echo();
            if obj2.value() > 1000 {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
   let obj3 = obj.clone();
   let th2 = std::thread::spawn(move || {
        obj3.echo();
        loop {
            println!("THREAD 2 -> ");
            obj3.inc();
            obj3.echo();
            if obj3.value() > 3000 {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    th1.join().expect("on join");
    th2.join().expect("on join");
    println!("value : {}", obj.value());

}