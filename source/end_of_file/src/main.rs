use clap::{App, Arg};
use std::io::{Seek, SeekFrom, Read};

struct Arguments {
    file: String,
    bytes: i64,
}

fn parse_args() -> Arguments {
    let m = App::new("end_of_file")
        .author("jared")
        .help("end of file")
        .version("0.1.0")
        .arg(
            Arg::with_name("bytes")
                .required(true)
                .multiple(false)
                .long("bytes")
                .takes_value(true)
                .short('b')
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("file")
                .required(true)
                .multiple(false)
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .get_matches();

    let f = m.value_of_lossy("file").unwrap();
    let b = m.value_of_lossy("bytes").unwrap().parse().unwrap();

    Arguments {
        file: f.to_string(),
        bytes: b,
    }
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    let f = std::fs::File::open(args.file)?;
    let length = f.metadata().unwrap().len();
    let mut r = std::io::BufReader::new(f);
    r.seek(SeekFrom::Start(length-args.bytes as u64))?;
    loop {
        let mut buf = [0; 1024];
        let l = r.read(&mut buf)?;
        if l == 0 {
            break;
        } else {
            for i in 0..l {
                print!("{}", buf[i] as char);
            }
        }
    }
    println!();
    Ok(())
}
