use clap::{App, Arg};

use filter::filter::{Filter, FilterImage};

struct Arguments {
    filename: String,
    output: String,
    index: usize,
    depth: usize,
}

fn parse_args() -> Arguments {
    let matches = App::new("filter")
        .about("Filter image example")
        .author("Jared Bruni")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("input file")
                .multiple(false)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("index")
                .help("filter index")
                .short('i')
                .long("index")
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true)
        )
        .arg(
            Arg::with_name("depth")
                .help("Filter Depth")
                .short('d')
                .long("depth")
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true)
        )
        .arg(
            Arg::with_name("output")
                .help("output image")
                .short('o')
                .long("output")
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true)
        )
        .get_matches();

    let f = matches.value_of_lossy("file").unwrap();
    let o = matches.value_of_lossy("output").unwrap();
    let ind = matches.value_of_lossy("index").unwrap().parse().unwrap();
    let dept = matches.value_of_lossy("depth").unwrap().parse().unwrap();

    Arguments {
        filename: f.to_string(),
        output: o.to_string(),
        index: ind,
        depth: dept,
    }
}

struct SelfAlphaBlend {}


impl Filter for SelfAlphaBlend {

    fn proc_filter(&mut self, im: &mut FilterImage, depth: usize) {

    }
}

fn proc_image(im: &mut FilterImage, filter: usize, depth: usize) {


}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    let mut image_file = FilterImage::load_from_png(&args.filename);
    proc_image(&mut image_file, args.index, args.depth);
    image_file.save_to_file(&args.output);
    Ok(())
}
