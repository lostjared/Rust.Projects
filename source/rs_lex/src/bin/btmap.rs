use clap::{App, Arg};
use rs_lex::rlex::map::rs_map::*;
use std::collections::BTreeMap;

/// Actions
enum Actions {
    Save,
    Load,
    Remove,
    Display
}

/// Arguments
struct Arguments {
    file: String,
    cls: String,
    key: String,
    value: String,
    action: Actions,
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
            Arg::with_name("class")
                .required(true)
                .multiple(false)
                .short('c')
                .long("class")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("key")
                .short('k')
                .long("key")
                .required(false)
                .default_value("list")
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
        .arg(
            Arg::with_name("list")
                .short('l')
                .long("list")
                .required(false),
        )
        .get_matches();
    let filename = m.value_of_lossy("file").unwrap();
    let class_name = m.value_of_lossy("class").unwrap();
    let key_value = m.value_of_lossy("key").unwrap();
    let value_value = m.value_of_lossy("value").unwrap();
    let mut action_value = if value_value == "<NO-VAL>" { Actions::Load } else { Actions::Save };
    if m.is_present("remove") {
        action_value = Actions::Remove;
    }
    if m.is_present("list") {
        action_value = Actions::Display;
    }
    Arguments {
        file: filename.to_string(),
        cls: class_name.to_string(),
        key: key_value.to_string(),
        value: value_value.to_string(),
        action: action_value,
    }
}
fn main() -> std::io::Result<()> {
    let mut args = parse_args();

    if args.key == "list" {
        args.action = Actions::Display;
    }

    match args.action {
        Actions::Load => {
            let f = std::fs::File::open(args.file)?;
            let r = std::io::BufReader::new(f);
            let mut btmap: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
            read_tree_map(r, &mut btmap);
            if btmap.contains_key(&args.cls) {
                let m = &btmap[&args.cls];
                if m.contains_key(&args.key) {
                    println!("Value is: {}", m[&args.key]);
                }
            }
        }
        Actions::Save => {
            let mut map: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
            if std::path::Path::new(&args.file).exists() {
                let f = std::fs::File::open(args.file.to_owned())?;
                let r = std::io::BufReader::new(f);
                read_tree_map(r, &mut map);
            }
            let m = map.get_mut(&args.cls);
            match m {
                Some(m) => {
                    m.insert(args.key, args.value);
                    let f = std::fs::File::create(args.file.to_owned())?;
                    let w = std::io::BufWriter::new(f);
                    save_tree_map(w, &map);
                }
                None => {
                    let mut mv : BTreeMap<String,String> = BTreeMap::new();
                    mv.insert(args.key, args.value.to_owned());
                    map.insert(args.cls, mv);
                    let f = std::fs::File::create(args.file.to_owned())?;
                    let w = std::io::BufWriter::new(f);
                    save_tree_map(w, &map);
                }
            }
            println!("Wrote to {}", args.file);
        }
        Actions::Remove => {
            let mut map: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
            let f = std::fs::File::open(args.file.to_owned())?;
            let r = std::io::BufReader::new(f);
            read_tree_map(r, &mut map);
            if map.contains_key(&args.cls) {
                let m = map.get_mut(&args.cls).unwrap();
                m.remove(&args.key);
                println!("rmeoved key: {}", args.key);
            } else {
                println!("could not find key: {}", args.key);
            }
            let f = std::fs::File::create(args.file.to_owned())?;
            let w = std::io::BufWriter::new(f);
            save_tree_map(w, &map);
        }
        Actions::Display => {
            let f = std::fs::File::open(args.file)?;
            let r = std::io::BufReader::new(f);
            let mut map: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
            read_tree_map(r, &mut map);
            let m = &map[&args.cls];
            for (key, value) in m {
                println!("{} = {}", key, value);
            }
        }
    }
    Ok(())
}
