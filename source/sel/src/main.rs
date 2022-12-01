use clap::{App, Arg};
use std::io::Read;

struct Arguments {
    mid: bool,
    r: String,
}

fn parse_args() -> Arguments {
    let m = App::new("sel")
        .arg(
            Arg::new("mid")
                .long("mid")
                .short('m')
                .takes_value(false)
                .required(false)
                .default_value("0"),
        )
        .arg(
            Arg::new("r")
                .long("regex")
                .short('r')
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .get_matches();
    let mid_ = m.is_present("mid");
    let reg = m.value_of_lossy("r").unwrap();
    Arguments {
        mid: mid_,
        r: reg.to_string(),
    }
}

fn main() -> std::io::Result<()> {
        let args = parse_args();
        let val = args.r;
        let sep = val.find(',');

        if sep == None {
            panic!("Missing required argument , ");
        }

        let mut string_value: String = String::new();
        let mut reader = std::io::stdin().lock();
        reader.read_to_string(&mut string_value).expect("on read");

        let sep = sep.unwrap();
        let num1 = &val[0..sep];
        let num2 = &val[sep + 1..];

        let start_pos: usize = if num1 == "$" {
            0
        } else {
            num1.parse().unwrap()
        };

        let stop_pos: usize = if num2 == "$" {
            string_value.len()
        } else {
            num2.parse().unwrap()
        };

        if start_pos < string_value.len() && stop_pos > start_pos {
            let cut_value = &string_value[start_pos..stop_pos];
            println!("{}", cut_value);
        }
       Ok(())
}
