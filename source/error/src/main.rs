#[derive(Debug)]
enum ErrorType {
    Type1,
    Type2
}

fn return_value(x: u8) -> Result<ErrorType, &'static str> {
    match x {
        0 => { Ok(ErrorType::Type1) }
        1 => { Ok(ErrorType::Type2) }
        _  => {
            Err("Not valid type")
        }
    }
}

fn main() {
    match return_value(1) {
        Ok(e) => {
            println!("Returned value: {:?}", e);
        }
        Err(e) => {
            println!("Error type: {}", e);
        }
    }

}
