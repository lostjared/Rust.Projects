extern crate image;

use std::env;
use image::io::Reader as ImageReader;
use image::{GenericImage, GenericImageView};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error ... requires filename");
        return Ok(());
    }
    let mut img = ImageReader::open(args.get(1).unwrap())?.decode().unwrap();
    let alpha : f32 = 0.5;
    for z in 0..img.height() {
        for i in 0..img.width() {
            let mut pixel = img.get_pixel(i, z);
            pixel[0] = ((pixel[0] as f32 * 0.5) + (i as f32 * alpha)) as u8;
            pixel[1] = ((pixel[1] as f32 * 0.5) + (z as f32 * alpha)) as u8;
            img.put_pixel(i,z, pixel);
        }
    }
    img.save("output.png").expect("wrote file");
    Ok(())
}