fn read_and_output(input: &str, output: &str) {
    // read
    let decoder = png::Decoder::new(std::fs::File::open(input).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];
    let _in_animation = reader.info().frame_control.is_some();

    // write
    let path = std::path::Path::new(output);
    let file = std::fs::File::create(path).unwrap();
    let ref mut w = std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, info.width, info.height); 
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&bytes).unwrap(); 
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 3 {
        read_and_output(args.get(1).unwrap(), args.get(2).unwrap());
    }
}
