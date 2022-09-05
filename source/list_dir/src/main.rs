use clap::{App, Arg};
use walkdir::WalkDir;

struct Arguments {
    paths: Vec<String>,
    dir: bool,
    files: bool,
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
                .default_value(".")
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
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("print files")
                .takes_value(false)
                .long("file")
                .short('f'),
        )
        .get_matches();
    let p: Vec<String> = m.values_of_lossy("paths").unwrap();
    let b = m.is_present("dir");
    let f = m.is_present("files");
    Arguments {
        paths: p,
        dir: b,
        files: f,
    }
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    for i in &args.paths {
        for entry in WalkDir::new(i) {
            match entry {
                Ok(entry) => {
                    if !args.dir && !args.files {
                        println!("{}", entry.path().display());
                    } else {
                        if args.dir && entry.file_type().is_dir() {
                            println!("{}", entry.path().display());
                        }
                        if args.files && entry.file_type().is_file() {
                            println!("{}", entry.path().display());
                        }
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
