// manipulate pixels
extern crate image;
use std::env;
use image::io::Reader as ImageReader;
use image::{GenericImage, GenericImageView};
use rayon::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error ... requires filename");
        return Ok(());
    }
    let img = ImageReader::open(args.get(1).unwrap())?.decode().unwrap();
    let mut bytes : Vec<u8>  = Vec::from(img.as_bytes());
    let values : Vec<(usize, &mut [u8])> = bytes.chunks_mut(img.width() as usize).enumerate().collect();
    values.into_par_iter().for_each(|(u, b)| {
        for pix in b {
            *pix = ((u as f32 * 0.2) + (0.5 * *pix as f32)) as u8;
        }
    });
    image::save_buffer("output.png", &mut bytes , img.width(), img.height(), image::ColorType::Rgba8).unwrap();
    Ok(())
}