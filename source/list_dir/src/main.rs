use clap::{App, Arg};
use walkdir::WalkDir;

struct Arguments {
    paths: Vec<String>,
    dir: bool,
}

fn parse_args() -> Arguments {
    let m = App::new("list_dir")
        .author("Jared")
        .help("List paths")
        .version("0.1.0")
        .arg(
            Arg::with_name("paths")
                .value_name("FILE_PATH")
                .help("input path(s)")
                .multiple(true)
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("dir")
                .value_name("DIR")
                .help("print dirs")
                .takes_value(false)
                .long("dir")
                .short('d'),
        )
        .get_matches();
    let p: Vec<String> = m.values_of_lossy("paths").unwrap();
    let b = m.is_present("dir");
    Arguments { paths: p, dir: b }
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    for i in &args.paths {
        for entry in WalkDir::new(i) {
            match entry {
                Ok(entry) => {
                    if args.dir && entry.file_type().is_dir() {
                        println!("{}", entry.path().display());
                    } 
                   
                    if !args.dir {
                        println!("{}", entry.path().display());
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
    Ok(())
}
