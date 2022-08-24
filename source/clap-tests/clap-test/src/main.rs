use clap::{App, Arg};

fn main() -> std::io::Result<()> {
    let matches = App::new("clap-test")
        .version("0.1.0")
        .author("Jared Bruni")
        .about("Clap Test")
        .arg(
            Arg::with_name("print_hello")
                .short('p')
                .long("print_hello")
                .help("Print hello")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("say")
                .short('s')
                .long("say")
                .help("say")
                .value_name("SAY")
                .takes_value(true)
        )
        .get_matches();

    if matches.is_present("print_hello") {
        println!("Hello, World!");
    }
    let text = matches.value_of("say");
    if text != None {
        let t = text.unwrap();
        println!("{}", t);
    }
    println!("{:?}", matches);
    Ok(())
}
