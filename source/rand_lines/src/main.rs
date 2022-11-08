use clap::{App, Arg};

struct Arguments {
    input: String,
    output: String,
    num: usize,
    len: usize,
}

fn parse_args() -> Arguments {
    let m = App::new("rand_lines")
        .help("random lines")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("input file")
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("output file")
                .takes_value(true)
                .required(false)
                .default_value("<STDOUT>")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .help("number of lines")
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("len")
                .short('l')
                .long("len")
                .help("length of lines")
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .get_matches();

    let input_ = m.value_of_lossy("input").unwrap();
    let output_ = m.value_of_lossy("output").unwrap();
    let num_ = m.value_of_lossy("num").unwrap().parse().unwrap();
    let len_ = m.value_of_lossy("len").unwrap().parse().unwrap();

    Arguments {
        input: input_.to_string(),
        output: output_.to_string(),
        num: num_,
        len: len_,
    }
}

fn main() -> std::io::Result<()> {
    Ok(())
}
