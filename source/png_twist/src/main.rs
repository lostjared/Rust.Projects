use image;
use image::GenericImageView;
use image::ImageBuffer;
use image::Pixel;

fn process_image(input: &str, output_file: &str, id: u8) {
    let img = image::open(input).expect("File not found!");
    let (w, h) = img.dimensions();
    let mut output = ImageBuffer::new(w, h);
    for (x, y, pixel) in img.pixels() {
        output.put_pixel(x, y, pixel.map(|p| p.wrapping_mul(id).wrapping_add(p ^ 15)));
    }
    output.save(output_file).expect("on save");
    println!("Wrote to file: {}", output_file);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 4 {
        let index = args.get(3).unwrap().parse().unwrap();
        process_image(args.get(1).unwrap(), args.get(2).unwrap(), index);
    } else {
        println!("{}: input.png output.png level_index", args.get(0).unwrap());
    }
}
