use clap::{App, Arg};
use rs_lex::rlex::*;
use std::io::Read;
use rand::Rng;

struct Arguments {
    file: String,
    num_words: usize,
    word_len: usize,
}

fn parse_args() -> Arguments {
    let m = App::new("code2text")
        .help("code2text")
        .about("jared bruni")
        .arg(
            Arg::with_name("input")
                .long("input")
                .short('i')
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("num")
                .long("num")
                .short('n')
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("len")
                .long("len")
                .short('l')
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .get_matches();

    let i = m.value_of_lossy("input").unwrap();
    let num = m.value_of_lossy("num").unwrap().parse().unwrap();
    let l = m.value_of_lossy("len").unwrap().parse().unwrap();
    Arguments {
        file: i.to_string(),
        num_words: num,
        word_len: l,
    }
}

fn gen_words(input: &str, num: usize, num_len: usize) {
    let f = std::fs::File::open(input).expect("on file open");
    let mut r = std::io::BufReader::new(f);
    let mut s = String::new();
    r.read_to_string(&mut s).expect("on read");
    let scan : Scanner = Scanner::new(&s);
    let mut v : Vec<String> = Vec::new();
    for i in scan {
        match i.get_type() {
            TokenType::Identifier => {
                let s = i.get_string();
                if s.len() > num_len {
                    let f = s.find('_');
                    if f != None {
                        let value2 = &s[..f.unwrap()];

                        let found_value = v.iter().find(|&x| *x == value2.to_string());
                        if found_value == None {
                            v.push(value2.to_string());
                        }
                    }
                }
                if v.len() > num {
                    break;
                }
            }
            _ => {}
        }
    }

    if v.len() < num {
        panic!("Not enough words");
    }

    let mut rng = rand::thread_rng();
    for _i in 0..num {
        let r = rng.gen_range(0..v.len());
        let value = v.get(r).clone().unwrap();
        print!("{} ", value);
        v.remove(r);
    }
    print!("\n");
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    gen_words(&args.file, args.num_words, args.word_len);
    Ok(())
}
