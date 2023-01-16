/// use
/// -i inputfile
/// -o outputfile
/// -r resoultion ex: 1920x1080
use clap::{App, Arg};

struct Arguments {
    infile: String,
    outfile: String,
    size_val: (u32, u32),
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
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("output")
                .help("output file")
                .short('o')
                .long("output")
                .takes_value(true)
                .required(true)
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
        .get_matches();

    let input_value = m.value_of_lossy("input").unwrap();
    let output_value = m.value_of_lossy("output").unwrap();
    let res = m.value_of_lossy("res").unwrap();

    let f = res.find('x').unwrap();
    let sx = &res[..f];
    let sy = &res[f + 1..];
    let size_value = (sx.parse().unwrap(), sy.parse().unwrap());

    Arguments {
        infile: input_value.to_string(),
        outfile: output_value.to_string(),
        size_val: size_value,
    }
}

/// main function
fn main() -> std::io::Result<()> {
    let args = parse_args();
    let i = image::open(&args.infile).unwrap();
    let resized = i.resize(args.size_val.0, args.size_val.1, image::imageops::Lanczos3);
    resized.save(&args.outfile).expect("Error on save");
    println!(
        "{} -> {} : {}x{}",
        args.infile, args.outfile, args.size_val.0, args.size_val.1
    );
    Ok(())
}
