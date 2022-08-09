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

    let mut input : String = String::new();
    let mut output : String = String::new();
    let mut index : u8 = 0;
    let count = opt::argz::getopt(&args, "i:o:n:", |i: char, param: String| {
        match i {
            'i' => {
                input = param.clone();
            }
            'o' => {
                output = param.clone();
            }
            'n' => {
                index = param.parse().unwrap();
            }
            _ => {

            }
        }
    });

    if count == 3 && !input.is_empty() && !output.is_empty() {
        process_image(&input, &output, index);
    } else {
        println!("{}: -i input.png -o output.png -n level_index", args.get(0).unwrap());
    }
}
