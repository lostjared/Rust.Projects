/*
code2text - generate random words from text files or source code

use:

    -i filename.txt
    -u only find underscore words
    -n how many words to find
    -l how long each word must be
    -m maximum length each word can be
    -o output to filename
*/

use clap::{App, Arg};
use colored::Colorize;
use logger::log::*;
use rand::Rng;
use rayon::prelude::*;
use rs_lex::rlex::*;
use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::sync::{Arc, Mutex};
struct Arguments {
    file: String,
    ofile: String,
    log: String,
    num_words: usize,
    word_len: usize,
    word_max: i32,
    under: bool,
    contains: String,
}

fn parse_args() -> Arguments {
    let m = App::new("code2text")
        .help("code2text - this program generates random words from text files.")
        .about("code2text - generate random words from code/text")
        .version("0.1.0")
        .author("Jared Bruni (jaredbruni@protonmail.com)")
        .arg(
            Arg::with_name("input")
                .long("input")
                .short('i')
                .takes_value(true)
                .required(false)
                .default_value("<STDIN>")
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
            Arg::with_name("output")
                .long("output")
                .short('o')
                .takes_value(true)
                .required(false)
                .default_value("<STDOUT>")
                .help("output to filename")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("log")
                .long("log")
                .short('g')
                .takes_value(true)
                .required(false)
                .default_value("log.txt")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("contains")
                .long("contains")
                .short('c')
                .takes_value(true)
                .required(false)
                .default_value("<NULL>")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("max")
                .long("max")
                .short('m')
                .takes_value(true)
                .required(false)
                .default_value("-1")
                .allow_invalid_utf8(true),
        )
        .get_matches();

    let i = m.value_of_lossy("input").unwrap();
    let num = m.value_of_lossy("num").unwrap().parse().unwrap();
    let l = m.value_of_lossy("len").unwrap().parse().unwrap();
    let outf = m.value_of_lossy("output").unwrap();
    let u = m.is_present("under");
    let log_file = m.value_of_lossy("log").unwrap();
    let cont = m.value_of_lossy("contains").unwrap();
    let max = m.value_of_lossy("max").unwrap().parse().unwrap();
    Arguments {
        file: i.to_string(),
        ofile: outf.to_string(),
        log: log_file.to_string(),
        num_words: num,
        word_len: l,
        word_max: max,
        under: u,
        contains: cont.to_string(),
    }
}

fn remove_chars(input: String) -> String {
    let mut new_value = String::new();
    for i in input.chars() {
        match i {
            '\'' | '\"' | '.' => {
                new_value.push(' ');
            }
            _ => {
                new_value.push(i);
            }
        }
    }
    new_value
}

fn gen_words<T>(
    r: T,
    num: usize,
    num_len: usize,
    num_max: i32,
    under: bool,
    contains: &str,
    ofile: &str,
    log_file: &str,
) where
    T: BufRead + Sized,
{
    let mut lines: Vec<String> = Vec::new();
    let mut log = Log::new_file_log("code2text", log_file, true, true);
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

    log.o("Scanning started");

    lines.into_par_iter().for_each(|line| {
        let mut scan: Scanner = Scanner::new(&remove_chars(line));
        loop {
            let token_result = scan.scan_token();
            match token_result {
                ScanResult::Error => {
                    eprintln!("code2text: Scanner error");
                    break;
                }
                ScanResult::Ok(val1) => match val1 {
                    Some(i) => {
                        if i.get_type() == TokenType::Identifier {
                            let mut v = data.lock().unwrap();
                            let mut map = map_data.lock().unwrap();
                            let s = i.get_string();
                            if s.len() > num_len && (num_max != -1 && s.len() <= num_max as usize) {
                                if map.contains_key(&s.to_string()) {
                                    continue;
                                } else {
                                    map.insert(s.to_string(), true);
                                    if !under {
                                        if contains == "<NULL>" || s.find(&contains) != None {
                                            v.push(s.to_string());
                                        }
                                        continue;
                                    }
                                    let f = s.find('_');
                                    if f != None {
                                        let value2 = &s[..f.unwrap()];
                                        v.push(value2.to_string());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        break;
                    }
                },
            }
        }
    });

    let mut v = data.lock().unwrap();

    println!("{}", "output: ".red());

    log.o(&format!(
        "Scanning finished.\n{{\n\tGathered {} tokens for pool.\n\tText contained {} lines.\n\tNow generating words...\n}}\n",
        v.len(),
        num_lines
    ));

    if v.len() < num {
        log.f("Not enough words");
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
        log.o(&format!("code2text: wrote to file: {}", ofile));
    }
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    if args.file != "<STDIN>" {
        gen_words(
            std::io::BufReader::new(std::fs::File::open(&args.file).unwrap()),
            args.num_words,
            args.word_len,
            args.word_max,
            args.under,
            &args.contains,
            &args.ofile,
            &args.log,
        );
    } else {
        gen_words(
            std::io::stdin().lock(),
            args.num_words,
            args.word_len,
            args.word_max,
            args.under,
            &args.contains,
            &args.ofile,
            &args.log,
        );
    }
    Ok(())
}
