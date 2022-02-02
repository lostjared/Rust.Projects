
#[derive(Debug)]
enum ErrorType {
    Type1,
    Type2
}

// return value is either Ok or Err.
fn return_value(x: u8) -> Result<ErrorType, &'static str> {
    match x {
        0 => { Ok(ErrorType::Type1) }
        1 => { Ok(ErrorType::Type2) }
        _  => {
            Err("Not valid type")
        }
    }
}

fn does_it_fail() -> std::io::Result<()> {
    std::fs::File::open("test.txt")?;
    Ok(())
}



fn main() -> std::io::Result<()> {
    let mut input_string = String::new();
    println!("input 0 for type1 or 1 for type2");
    std::io::stdin().read_line(&mut input_string).expect("on readline");
    let input_type: u8 = input_string.trim().parse::<u8>().unwrap();
    match return_value(input_type) {
        Ok(e) => {
            println!("Returned value: {:?}", e);
        }
        Err(e) => {
            println!("Error type: {}", e);
        }
    }
    does_it_fail()?;
    Ok(())
}
