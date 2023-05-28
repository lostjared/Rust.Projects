use clap::{App, Arg};
use rs_lex::rlex::map::rs_map::{read_map, save_map};
use std::collections::HashMap;

/// Arguments
struct Arguments {
    file: String,
    key: String,
    value: String,
    action: u8,
}

/// parse the arguments
fn parse_args() -> Arguments {
    let m = App::new("map")
        .author("Jared")
        .help("map edit")
        .about("map")
        .version("0.1.0")
        .arg(
            Arg::with_name("file")
                .required(true)
                .multiple(false)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("key")
                .short('k')
                .long("key")
                .required(true)
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("value")
                .short('v')
                .long("value")
                .takes_value(true)
                .default_value("<NO-VAL>")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("remove")
                .short('r')
                .long("remove")
                .required(false),
        )
        .get_matches();
    let filename = m.value_of_lossy("file").unwrap();
    let key_value = m.value_of_lossy("key").unwrap();
    let value_value = m.value_of_lossy("value").unwrap();
    let mut action_value = if value_value == "<NO-VAL>" { 0u8 } else { 1u8 };
    if m.is_present("remove") {
        action_value = 2u8;
    }
    Arguments {
        file: filename.to_string(),
        key: key_value.to_string(),
        value: value_value.to_string(),
        action: action_value,
    }
}

/// main function
fn main() -> std::io::Result<()> {
    let args = parse_args();
    match args.action {
        0u8 => {
            let mut map: HashMap<String, String> = HashMap::new();
            read_map(&args.file, &mut map)?;
            if map.contains_key(&args.key) {
                println!("Value: {}", map[&args.key]);
            } else {
                println!("Does not contain key: {}", args.key);
            }
        }
        1u8 => {
            let mut map: HashMap<String, String> = HashMap::new();
            if std::path::Path::new(&args.file).exists() {
                read_map(&args.file, &mut map)?;
            }
            map.insert(args.key, args.value);
            save_map(&args.file, &map)?;
            println!("Wrote to {}", args.file);
        }
        2u8 => {
            let mut map: HashMap<String, String> = HashMap::new();
            read_map(&args.file, &mut map)?;
            if map.contains_key(&args.key) {
                map.remove(&args.key);
                println!("rmeoved key: {}", args.key);
            } else {
                println!("could not find key: {}", args.key);
            }
            save_map(&args.file, &map)?;
            println!("Wrote to {}", args.file);
        }
        _ => {}
    }

    Ok(())
}
