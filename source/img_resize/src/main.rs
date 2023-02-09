/// use
/// -i inputfile
/// -o outputfile
/// -r resoultion ex: 1920x1080
/// optional:
/// -e for exact resize
use clap::{App, Arg};
use colored::Colorize;

struct Arguments {
    infile: String,
    outfile: String,
    size_val: (u32, u32),
    exact: bool,
    list: String,
}
/// parse arguments
fn parse_args() -> Arguments {
    let m = App::new("img_resize")
        .help("resize an image")
        .author("Jared Bruni")
        .arg(
            Arg::new("input")
                .help("input file")
                .short('i')
                .long("input")
                .takes_value(true)
                .required(true)
                .conflicts_with("list")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("output")
                .help("output file")
                .short('o')
                .long("output")
                .takes_value(true)
                .conflicts_with("list")
                .required(true)
                .default_value("<NULL>")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("res")
                .help("Resolution WidthxHeight")
                .short('r')
                .long("res")
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("exact")
                .help("exact resize")
                .short('e')
                .long("exact")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::new("list")
                .help("process list")
                .short('l')
                .long("list")
                .takes_value(true)
                .required(false)
                .default_value("<NULL>")
                .allow_invalid_utf8(true),
        )
        .get_matches();
    let ls = m.value_of_lossy("list").unwrap();

    if ls.to_string() != "<NULL>" {
        let res = m.value_of_lossy("res").unwrap();
        let f = res.find('x').unwrap();
        let sx = &res[..f];
        let sy = &res[f + 1..];
        let size_value = (sx.parse().unwrap(), sy.parse().unwrap());
        let exact_value = m.is_present("exact");
        Arguments {
            infile: "<NULL>".to_string(),
            outfile: "<NULL>".to_string(),
            size_val: size_value,
            exact: exact_value,
            list: ls.to_string(),
        }
    } else {
        let input_value = m.value_of_lossy("input").unwrap();
        let output_value = m.value_of_lossy("output").unwrap();
        let res = m.value_of_lossy("res").unwrap();
        let f = res.find('x').unwrap();
        let sx = &res[..f];
        let sy = &res[f + 1..];
        let size_value = (sx.parse().unwrap(), sy.parse().unwrap());
        let exact_value = m.is_present("exact");
        Arguments {
            infile: input_value.to_string(),
            outfile: output_value.to_string(),
            size_val: size_value,
            exact: exact_value,
            list: ls.to_string(),
        }
    }
}

fn build_list(input: &str) -> Vec<String> {
    let data = std::fs::read_to_string(input).unwrap();
    let mut v: Vec<String> = Vec::new();
    for file in data.lines() {
        v.push(file.to_string());
    }
    v
}

fn convert_file(infile: &str, outfile: &str, size_val: (u32, u32), exact: bool) {
    let i = image::open(infile).unwrap();
    let resized = if exact {
        i.resize_exact(size_val.0, size_val.1, image::imageops::Lanczos3)
    } else {
        i.resize(size_val.0, size_val.1, image::imageops::Lanczos3)
    };
    resized.save(outfile).expect("Error on save");
    println!(
        "{} -> {} : {}x{}",
        infile.red(),
        outfile.blue(),
        resized.width(),
        resized.height()
    );
}

/// main function
fn main() -> std::io::Result<()> {
    let args = parse_args();
    if args.list == "<NULL>" {
        convert_file(&args.infile, &args.outfile, args.size_val, args.exact);
    } else {
        let ls = build_list(&args.list);
        for i in ls {
            let ext = std::path::Path::new(&i).extension().unwrap();

            let path = i.find(&ext.to_string_lossy().to_string()).unwrap();
            let lpath = &i[..path];
            let new_f = format!(
                "{}{}x{}.{}",
                lpath,
                args.size_val.0,
                args.size_val.1,
                ext.to_string_lossy()
            );
            convert_file(&i, &new_f, args.size_val, args.exact);
        }
    }
    Ok(())
}
