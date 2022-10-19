use clap::{App, Arg};
use rand::Rng;
use rs_lex::rlex::*;
use std::collections::HashMap;
use std::io::Read;

struct Arguments {
    file: String,
    num_words: usize,
    word_len: usize,
    under: bool,
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
        .arg(
            Arg::with_name("under")
                .long("underscore")
                .short('u')
                .takes_value(false)
                .required(false),
        )
        .get_matches();

    let i = m.value_of_lossy("input").unwrap();
    let num = m.value_of_lossy("num").unwrap().parse().unwrap();
    let l = m.value_of_lossy("len").unwrap().parse().unwrap();
    let u = m.is_present("under");
    Arguments {
        file: i.to_string(),
        num_words: num,
        word_len: l,
        under: u,
    }
}

fn gen_words(input: &str, num: usize, num_len: usize, under: bool) {
    let f = std::fs::File::open(input).expect("on file open");
    let mut r = std::io::BufReader::new(f);
    let mut s = String::new();
    r.read_to_string(&mut s).expect("on read");
    let slen = s.len();
    let mut scan: Scanner = Scanner::new(&s);
    let mut v: Vec<String> = Vec::new();
    let mut counter = 0;

    let mut map: HashMap<String, bool> = HashMap::new();
    loop {
        let token_result = scan.scan_token();
        match token_result {
            ScanResult::Error => {}
            ScanResult::Ok(val1) => {
                match val1 {
                    Some(i) => {
                        if counter % 1000 == 0 {
                            let per: f64 = (scan.getpos() as f64 / slen as f64) * 100.0;

                            println!(
                                "{} - ({}/{}) {:.2}%  tokens processed...",
                                counter,
                                scan.getpos(),
                                slen,
                                per
                            );
                        }
                        counter += 1;
                        match i.get_type() {
                            TokenType::Identifier => {
                                let s = i.get_string();
                                if s.len() > num_len {
                                    if map.contains_key(&s.to_string()) {
                                        continue;
                                    } else {
                                        map.insert(s.to_string(), true);

                                        if under == false {
                                            v.push(s.to_string());
                                            continue;
                                        }
                                        let f = s.find('_');
                                        if f != None {
                                            let value2 = &s[..f.unwrap()];
                                            v.push(value2.to_string());
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    }
    println!(
        "code2text: scanning finish scanned {} tokens, generating words...",
        counter
    );
    if v.len() < num {
        panic!("Not enough words");
    }

    let mut rng = rand::thread_rng();
    for _i in 0..num {
        if v.is_empty() {
            break;
        }
        let r = rng.gen_range(0..v.len());
        let value = v.get(r).clone().unwrap();
        print!("{} ", value);
        v.remove(r);
    }
    print!("\n");
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    gen_words(&args.file, args.num_words, args.word_len, args.under);
    Ok(())
}
