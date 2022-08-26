


fn fill_buffer(buf: &mut [u8], pitch: usize, w: usize, h: usize) {
    for z in 0..h {
        for i in 0..w {
            let pos = z * pitch + (i*4);
            buf[pos] = 255;
            buf[pos+1] = 255;
            buf[pos+2] = 255;
            buf[pos+3] = 255;
        }
    }
}

fn write_buffer(output: &str,buf: &mut [u8], wx: usize, hx: usize) {
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
    let mut buffer = vec![0u8; w*h*4];
    let len = buffer.len();
    fill_buffer(&mut buffer[0..len], w*4, 640, 480);
    write_buffer("output.png", &mut buffer[0..len], 640, 480);
    println!("Wrote: output.png");
    Ok(())
}