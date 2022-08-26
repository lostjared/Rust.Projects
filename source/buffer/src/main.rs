


fn fill_buffer(buf: &mut [u8], pitch: usize, bpp: usize, w: usize, h: usize) {

    let mut num : f32 = 0.5;

    for z in 0..h {
        for i in 0..w {
            let pos = z * pitch + (i*bpp);
            buf[pos] = (i as f32 *num) as u8;
            buf[pos+1] = (z as f32 *num) as u8;
            buf[pos+2] = (z as f32 *num) as u8;
            buf[pos+3] = 255;
        }
    }
}

fn write_buffer(output: &str,buf: &[u8], wx: usize, hx: usize) {
    let path = std::path::Path::new(output);
    let file = std::fs::File::create(path).unwrap();
    let ref mut w = std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, wx as u32, hx as u32); 
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&buf).unwrap(); 
}


fn main() -> std::io::Result<()> {
    let w = 640;
    let h = 480;
    let bpp = 4;
    let mut buffer = vec![0u8; w*h*bpp];
    let len = buffer.len();
    fill_buffer(&mut buffer[0..len], w*bpp, bpp, 640, 480);
    write_buffer("output.png", &buffer[0..len], 640, 480);
    println!("Wrote: output.png");
    Ok(())
}