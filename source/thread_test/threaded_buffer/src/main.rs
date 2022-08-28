use rand::Rng;
use rayon::prelude::*;
use clap::{App, Arg};

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

pub fn chain_vector(v: Vec<&mut [u8]>) -> Vec<u8> {
    let mut final_bytes : Vec<u8> = Vec::new();
    for i in 0..v.len() {
        for z in 0..v[i].len() {
            final_bytes.push(v[i][z]);
        }
    }
    final_bytes
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

    let matches = App::new("threaded_buffer").help("gen pixels concurrently").arg( Arg::with_name("num").short('n').required(true).long("number").takes_value(true)).get_matches();
    let width = 1920;
    let height = 1080;
    let bpp = 4;
    let num_iterators : usize = matches.value_of("num").unwrap().parse().unwrap();
    let mut bytes : Vec<u8> = vec![0u8; width*height*bpp];
    let mut file_chunk : Vec<&mut [u8]> = bytes.chunks_mut(num_iterators).collect();
    file_chunk.par_iter_mut().for_each(|v| {
        process_chunk(v);
    });
    let final_bytes = chain_vector(file_chunk);
    let flen = final_bytes.len();
    save_to_file("output.png", &final_bytes[0..flen], width, height);
    println!("wrote to file: output.png");
}
