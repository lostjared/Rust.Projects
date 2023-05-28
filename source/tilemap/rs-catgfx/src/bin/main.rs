use clap::{App, Arg};
use rs_catgfx::catgfx::*;

/// program operation enum
enum Operation {
    Concat,
    Extract,
    List,
    About,
}

/// program arguments
struct Arguments {
    source: String,
    dir: Option<String>,
    input: Option<String>,
    mode: Operation,
}
/// parse arguments
fn parse_args() -> Arguments {
    let m = App::new("catgfx")
        .version("0.1.0")
        .author("jared")
        .arg(
            Arg::with_name("cat")
                .short('c')
                .long("cat")
                .takes_value(true)
                .required(false)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("extract")
                .short('e')
                .long("extract")
                .takes_value(true)
                .required(false)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("dir")
                .short('d')
                .long("dir")
                .takes_value(true)
                .required(false)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("input")
                .short('i')
                .long("input")
                .takes_value(true)
                .required(false)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("list")
                .short('l')
                .long("list")
                .takes_value(true)
                .required(false)
                .allow_invalid_utf8(true),
        )
        .get_matches();

    let value = m.value_of_lossy("list");
    if value != None {
        return Arguments {
            source: value.unwrap().to_string(),
            dir: None,
            input: None,
            mode: Operation::List,
        };
    }

    let value = m.value_of_lossy("extract");
    if value != None {
        let d = m.value_of_lossy("dir").unwrap();
        return Arguments {
            source: value.unwrap().to_string(),
            dir: Some(d.to_string()),
            input: None,
            mode: Operation::Extract,
        };
    }

    let value = m.value_of_lossy("cat");
    if value != None {
        let c = m.value_of_lossy("input").unwrap();
        return Arguments {
            source: value.unwrap().to_string(),
            dir: None,
            input: Some(c.to_string()),
            mode: Operation::Concat,
        };
    }

    Arguments {
        source: String::new(),
        mode: Operation::About,
        dir: None,
        input: None,
    }
}

/// main function
fn main() -> std::io::Result<()> {
    // parse argumetns store in args
    let args = parse_args();
    // match args.mode
    match args.mode {
        // print error no input
        Operation::About => {
            println!("error: no input");
        }
        // operation concat
        Operation::Concat => {
            let i = args.input.unwrap();
            cat_gfx(&args.source, &i)?;
        }
        // operation extract
        Operation::Extract => {
            let e = args.dir.unwrap();
            extract_gfx(&args.source, &e)?;
        }
        // operation list files
        Operation::List => {
            list_gfx(&args.source)?;
        }
    }
    Ok(())
}
