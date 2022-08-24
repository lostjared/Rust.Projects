use clap::{App, Arg};

#[derive(Debug)]
struct Arguments {
    pub text: String,
    pub upper: bool,
    pub files: Vec<String>
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
        ).arg(Arg::with_name("files").value_name("FILE").help("input file(s)").multiple(true).default_value(".").allow_invalid_utf8(true))

        .get_matches();

    let val_s = matches.value_of("data").unwrap();
    let val_b = matches.is_present("upper");
    let v = matches.values_of_lossy("files").unwrap();

    Arguments {
        text: val_s.to_string(),
        upper: val_b,
        files: v
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
    // print out filenames
    for i in &args.files {
       println!("{}", i);
    }
    print!("\n");
    Ok(())
}
