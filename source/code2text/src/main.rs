/*

code2text - generate random words from text files or source code

use:

    -i filename.txt
    -s stop when found enough words
    -u only find underscore words
    -n how many words to find
    -l how long each word must be
    -m collect at most max words
    -o output to filename

*/

use clap::{App, Arg};
use rand::Rng;
use rayon::prelude::*;
use rs_lex::rlex::*;
use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::sync::{Arc, Mutex};

struct Arguments {
    file: String,
    ofile: String,
    num_words: usize,
    word_len: usize,
    under: bool,
    stop: bool,
    max: usize,
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
                .allow_invalid_utf8(true)
                .help("input filename"),
        )
        .arg(
            Arg::with_name("num")
                .long("num")
                .short('n')
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true)
                .help("how many words to find"),
        )
        .arg(
            Arg::with_name("len")
                .long("len")
                .short('l')
                .takes_value(true)
                .required(true)
                .help("minimum length of the words")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("under")
                .long("underscore")
                .short('u')
                .takes_value(false)
                .help("should I find underscore words")
                .required(false),
        )
        .arg(
            Arg::with_name("stop")
                .long("stop")
                .short('s')
                .takes_value(false)
                .help("stop after num words gathered")
                .required(false),
        )
        .arg(
            Arg::with_name("max")
                .long("max")
                .short('m')
                .takes_value(true)
                .required(false)
                .default_value("0")
                .help("Gather max words before genenrating random words")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .short('o')
                .takes_value(true)
                .required(false)
                .default_value("<STDOUT>")
                .help("output to filename")
                .allow_invalid_utf8(true),
        )
        .get_matches();

    let i = m.value_of_lossy("input").unwrap();
    let num = m.value_of_lossy("num").unwrap().parse().unwrap();
    let l = m.value_of_lossy("len").unwrap().parse().unwrap();
    let max_t = m.value_of_lossy("max").unwrap().parse().unwrap();
    let outf = m.value_of_lossy("output").unwrap();
    let u = m.is_present("under");
    let s = m.is_present("stop");
    Arguments {
        file: i.to_string(),
        ofile: outf.to_string(),
        num_words: num,
        word_len: l,
        under: u,
        stop: s,
        max: max_t,
    }
}

fn remove_chars(input: String) -> String {
    let mut new_value = String::new();
    for i in input.chars() {
        match i {
            '\'' | '\"' | '.' => {
                new_value.push(' ');
            }
            _=> {
                new_value.push(i);
            }
        }
    }
    new_value
}

fn gen_words(
    input: &str,
    num: usize,
    num_len: usize,
    under: bool,
    stop: bool,
    max_t: usize,
    ofile: &str,
) {
    let f = std::fs::File::open(input).expect("on file open");
    let r = std::io::BufReader::new(f);
    let mut lines: Vec<String> = Vec::new();
    for line in r.lines() {
        match line {
            Ok(l) => {
                lines.push(l);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    let num_lines = lines.len();
    let v: Vec<String> = Vec::new();
    let map: HashMap<String, bool> = HashMap::new();

    let data = Arc::new(Mutex::new(v));
    let map_data = Arc::new(Mutex::new(map));

    lines.into_par_iter().for_each(|line| {
        let mut scan: Scanner = Scanner::new(&remove_chars(line));
        loop {
            let token_result = scan.scan_token();
            match token_result {
                ScanResult::Error => {
                    eprintln!("code2text: Scanner error");
                    break;
                }
                ScanResult::Ok(val1) => {
                    let mut v = data.lock().unwrap();
                    let mut map = map_data.lock().unwrap();

                    
                    match val1 {
                        Some(i) => {
                            if i.get_type() == TokenType::Identifier {
                                let s = i.get_string();
                                if s.len() > num_len {
                                    if map.contains_key(&s.to_string()) {
                                        continue;
                                    } else {
                                        map.insert(s.to_string(), true);
                                        if !under {
                                            v.push(s.to_string());
                                            if stop && v.len() > num {
                                                break;
                                            }
                                            if max_t != 0 && v.len() > max_t {
                                                break;
                                            }
                                            continue;                        
                                        }
                                        let f = s.find('_');
                                        if f != None {
                                            let value2 = &s[..f.unwrap()];
                                            v.push(value2.to_string());
                                            if stop && v.len() > num {
                                                break;
                                            }
                                            if max_t != 0 && v.len() > max_t {
                                                break;
                                            }
                                            continue;                        
                                        }
                                    }
                                }
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
        }
    });

    let mut v = data.lock().unwrap();

    println!(
        "code2text: scanning finished gathered {} tokens for pool out of {} lines, generating words...",
        v.len(),
        num_lines
    );
    if v.len() < num {
        panic!("Not enough words");
    }
    let mut w: std::io::BufWriter<Box<dyn Write>>;
    if ofile != "<STDOUT>" {
        let f = std::fs::File::create(ofile).unwrap();
        w = std::io::BufWriter::new(Box::new(f));
    } else {
        w = std::io::BufWriter::new(Box::new(std::io::stdout().lock()));
    }
    let mut rng = rand::thread_rng();
    for _i in 0..num {
        if v.is_empty() {
            break;
        }
        let r = rng.gen_range(0..v.len());
        let value = v.get(r).unwrap();
        write!(w, "{} ", value).expect("on write");
        v.remove(r);
    }
    writeln!(w).expect("on write");
    if ofile != "<STDOUT>" {
        println!("code2text: wrote to file: {}", ofile);
    }
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    gen_words(
        &args.file,
        args.num_words,
        args.word_len,
        args.under,
        args.stop,
        args.max,
        &args.ofile,
    );
    Ok(())
}
