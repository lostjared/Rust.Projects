// manipulate pixels
extern crate image;
use image::io::Reader as ImageReader;
use image::*;
use rayon::prelude::*;

fn main() -> std::io::Result<()> {

    let img = ImageReader::open("image.png")?.decode().unwrap();
    let mut bytes : Vec<u8>  = Vec::from(img.as_bytes());
    println!("{}x{}", img.width(), img.height());
    let values : Vec<(usize, &mut [u8])> = bytes.chunks_mut(img.width() as usize * 4 as usize).enumerate().collect();
    values.into_par_iter().for_each(|(_u, b)| {
        let mut y : usize = 0;
        let len = b.len()-1;
        while y < len {
            b[y+1] = ((y as f32 * 0.3 ) + (0.5 * b[y+1] as f32)) as u8;
            b[y+2] = (0.5 * b[y+2] as f32) as u8;
            y += 4;
        }
    });
    image::save_buffer("output.png", &mut bytes , img.width(), img.height(), image::ColorType::Rgba8).unwrap();
    Ok(())
}