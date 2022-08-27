use ::filter::filter::FilterImage;
use rand::Rng;
use rayon::prelude::*;

fn process_chunk(buf: &mut [u8]) {
    let mut rng = rand::thread_rng();
    let mut i = 0;
    while i < buf.len() {
        buf[i] = rng.gen_range(0..255);
        buf[i+1] = rng.gen_range(0..255);
        buf[i+2] = rng.gen_range(0..255);
        buf[i+3] = 255;
        i += 4;
    }
}

pub fn save_to_file(filename: &str, bytes: &[u8], width: usize, height: usize) {
    let path = std::path::Path::new(filename);
    let file = std::fs::File::create(path).unwrap();
    let w = &mut std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let len = bytes.len();
    writer.write_image_data(&bytes[0..len]).unwrap();
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let filename = arguments.get(1).unwrap();
    let mut im = FilterImage::load_from_png(filename);
    let mut file_chunk : Vec<&mut [u8]> = im.bytes.chunks_mut(8).collect();
    file_chunk.par_iter_mut().for_each(|v| {
        process_chunk(v);
    });
    let mut final_bytes : Vec<u8> = Vec::new();
    for i in 0..file_chunk.len() {
        for z in 0..file_chunk[i].len() {
            let v = file_chunk[i][z];
            final_bytes.push(v);
        }
    }
    let flen = final_bytes.len();
    save_to_file("output.png", &final_bytes[0..flen], im.width, im.height);
    println!("wrote to file: output.png");
}
