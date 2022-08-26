pub mod filter {

    pub struct FilterImage {
        pub bytes: Vec<u8>,
        pub width: usize,
        pub height: usize,
        pub bpp: usize,
    }

    impl FilterImage {
        pub fn load_from_png(filename: &str) -> Self {
            let decoder = png::Decoder::new(std::fs::File::open(filename).unwrap());
            let mut reader = decoder.read_info().unwrap();
            let mut buf = vec![0; reader.output_buffer_size()];
            let info = reader.next_frame(&mut buf).unwrap();
            let _in_animation = reader.info().frame_control.is_some();
            Self {
                bytes: buf,
                width: info.width as usize,
                height: info.height as usize,
                bpp: 4,
            }
        }

        pub fn save_to_file(&self, filename: &str) {
            let path = std::path::Path::new(filename);
            let file = std::fs::File::create(path).unwrap();
            let ref mut w = std::io::BufWriter::new(file);
            let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            let len = self.bytes.len();
            writer.write_image_data(&self.bytes[0..len]).unwrap();
        }
    }

    pub trait Filter {
        fn proc_filter(&mut self, im: &mut FilterImage, depth: f32);
    }
}
