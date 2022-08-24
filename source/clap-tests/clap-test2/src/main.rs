use clap::{App, Arg};

#[derive(Debug)]
struct Arguments {
    pub text: String,
    pub upper: bool,
}

fn parse_args() -> Arguments {
    let matches = App::new("clap-test2")
        .version("0.1.0")
        .author("Jared Bruni")
        .about("Clap Test2")
        .arg(
            Arg::with_name("upper")
                .short('u')
                .long("upper-case")
                .help("upper case string")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("data")
                .short('d')
                .long("data-string")
                .help("data to process")
                .value_name("DATA")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let val_s = matches.value_of("data").unwrap();
    let val_b = matches.is_present("upper");

    Arguments {
        text: val_s.to_string(),
        upper: val_b,
    }
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    for i in args.text.chars() {
        if args.upper {
            print!("{}", i.to_uppercase());
        } else {
            print!("{}", i.to_lowercase());
        }
    }
    print!("\n");

    Ok(())
}
