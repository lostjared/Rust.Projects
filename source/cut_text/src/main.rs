use clap::{App, Arg};

struct Arguments {
    file: String,
    start: usize,
    number: usize,
}

fn parse_args() -> Arguments {
    let m = App::new("cut_text")
        .author("Jared")
        .help("cut text")
        .version("0.1.0")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("input file(s)")
                .multiple(true)
                .default_value("<STDIN>")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("cut")
                .value_name("CUT")
                .help("cut start,number")
                .multiple(false)
                .allow_invalid_utf8(true)
                .required(true)
                .short('c')
                .long("cut"),
        )
        .get_matches();

    let f = m.value_of_lossy("file").unwrap();
    let s = m.value_of_lossy("cut").unwrap();

    let s: usize = 0;
    let n: usize = 0;

    Arguments {
        file: f.to_string(),
        start: s,
        number: n,
    }
}

fn cut_text(input: &str, start: usize, num: usize) -> String {
    let d = &input[start..start + num];
    String::from(d)
}

fn main() -> std::io::Result<()> {
    Ok(())
}
