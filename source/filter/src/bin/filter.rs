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
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("depth")
                .help("Filter Depth")
                .short('d')
                .long("depth")
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("output")
                .help("output image")
                .short('o')
                .long("output")
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true),
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
struct SelfScale {}

impl Filter for SelfAlphaBlend {
    fn proc_filter(&mut self, im: &mut FilterImage, depth: usize) {
        let len = im.bytes.len();
        let fdepth: f32 = 0.1 * depth as f32;
        let dep: u8 = fdepth as u8;
        let buf = &mut im.bytes[0..len];
        let pitch = im.width * im.bpp;
        for z in 0..im.height {
            for i in 0..im.width {
                let pos = z * pitch + (i * im.bpp);
                buf[pos] = buf[pos].wrapping_add(dep.wrapping_mul(buf[pos]));
                buf[pos + 1] = buf[pos + 1].wrapping_add(dep.wrapping_mul(buf[pos + 1]));
                buf[pos + 2] = buf[pos + 2].wrapping_add(dep.wrapping_mul(buf[pos + 2]));
                buf[pos + 3] = 255;
            }
        }
    }
}

impl Filter for SelfScale {
    fn proc_filter(&mut self, im: &mut FilterImage, depth: usize) {
        let len = im.bytes.len();
        let fdepth: f32 = 0.1 * depth as f32;
        let dep: u8 = fdepth as u8;
        let buf = &mut im.bytes[0..len];
        let pitch = im.width * im.bpp;
        for z in 0..im.height {
            for i in 0..im.width {
                let pos = z * pitch + (i * im.bpp);
                buf[pos] = dep.wrapping_mul(buf[pos]);
                buf[pos + 1] = dep.wrapping_mul(buf[pos + 1]);
                buf[pos + 2] = dep.wrapping_mul(buf[pos + 2]);
                buf[pos + 3] = 255;
            }
        }
    }
}

fn proc_image(im: &mut FilterImage, filter: &mut dyn Filter, depth: usize) {
    filter.proc_filter(im, depth);
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    let mut selfalpha = SelfAlphaBlend{};
    let mut selfscale = SelfScale{};
    let mut f_v  : Vec<&mut dyn Filter> = vec![&mut selfalpha, &mut selfscale];
    if args.index >= f_v.len() {
        println!("filter: Index out of range!");
        return Ok(());
    }
    let mut image_file = FilterImage::load_from_png(&args.filename);
    println!("filter: Filtering image: {}", args.filename);
    proc_image(&mut image_file, f_v[args.index], args.depth);
    image_file.save_to_file(&args.output);
    println!("filter: Wrote file: {}", args.output);
    Ok(())
}
