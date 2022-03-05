
 #[derive(Debug)]
enum ValueType {
    TYPE1,
    TYPE2,
    TYPE3(String),
}

fn to_value(t: ValueType) {
    match &t {
        ValueType::TYPE1 => {
            println!("type 1");
        }
        ValueType::TYPE2 => {
            println!("type 2");
        }
        ValueType::TYPE3(s) => {
            println!("type 3 {}", s);
        }
    }
    println!("{:?}", t);
}


fn main() {
    to_value(ValueType::TYPE1);
    to_value(ValueType::TYPE2);
    to_value(ValueType::TYPE3("test ".to_string()));
}
