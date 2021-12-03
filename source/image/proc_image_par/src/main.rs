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
        let mut x : usize = 0;
        let len = b.len()-1;
        while x < len {
            b[x+1] = ((x as f32 * 0.3 ) + (0.5 * b[x+1] as f32)) as u8;
            b[x+2] = (0.5 * b[x+2] as f32) as u8;
            x += 4;
        }
    });
    image::save_buffer("output.png", &mut bytes , img.width(), img.height(), image::ColorType::Rgba8).unwrap();
    Ok(())
}