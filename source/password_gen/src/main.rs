use rand::rngs::ThreadRng;
use rand::Rng;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;

#[derive(Clone, Copy, Debug)]
pub enum CharType {
    UpperLetter,
    LowerLetter,
    Digit,
    Symbol,
}

fn generate_char(rng: &mut ThreadRng, ch_type: &CharType) -> char {
    match ch_type {
        CharType::UpperLetter => {
            return rng.gen_range('A' as u8..='Z' as u8) as char;
        }
        CharType::LowerLetter => {
            return rng.gen_range('a' as u8..='z' as u8) as char;
        }
        CharType::Digit => {
            return rng.gen_range('0' as u8..='9' as u8) as char;
        }
        CharType::Symbol => {
            let valid = String::from("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~");
            let rpos = rng.gen_range(0..valid.len());
            return valid.chars().nth(rpos).unwrap();
        }
    }
}

fn generate_password(length: u32) -> String {
    let mut value: Vec<char> = Vec::new();
    let mut rng = rand::thread_rng();
    for _i in 0..length * 2 {
        let rtype = rng.gen_range(0..4);
        match rtype {
            0 => value.push(generate_char(&mut rng, &CharType::UpperLetter)),
            1 => value.push(generate_char(&mut rng, &CharType::LowerLetter)),
            2 => value.push(generate_char(&mut rng, &CharType::Digit)),
            3 => value.push(generate_char(&mut rng, &CharType::Symbol)),
            _ => value.push(generate_char(&mut rng, &CharType::UpperLetter)),
        }
    }
    let mut shuffle_rand = rand::thread_rng();
    let mut irs = Irs::default();
    irs.shuffle(&mut value, &mut shuffle_rand)
        .expect("on shuffle");
    let mut pw = String::new();
    for i in 0..length {
        pw.push(value[i as usize]);
    }
    pw
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("argument: provide length of password");
        std::process::exit(1);
    }
    let len = args[1].parse::<u32>().unwrap();
    let password = generate_password(len);
    println!("generated password: {}", password);
}
