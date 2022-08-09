use image;
use image::GenericImageView; 
use image::Pixel;
use image::ImageBuffer;

fn read_bytes_and_proc(input: &str, output_file: &str, id: u32) {
    let img = image::open(input).expect("File not found!");
    let (w, h) = img.dimensions();
    let mut output = ImageBuffer::new(w, h);
    let value = 20;
    for (x, y, pixel) in img.pixels() {
        output.put_pixel(x, y, 
            pixel.map(|p| p.wrapping_add(p^value))
        );
    }
    output.save(output_file).expect("on save");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 4 {
        let index = args.get(3).unwrap().parse().unwrap();
        read_bytes_and_proc(args.get(1).unwrap(), args.get(2).unwrap(), index);
    }
}
