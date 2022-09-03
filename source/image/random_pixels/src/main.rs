use rand::Rng;

fn main() {
    let mut random = rand::thread_rng();
    let mut img = image::ImageBuffer::new(1920, 1080);
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let r : u8 = random.gen();
        let g : u8 = random.gen();
        let b : u8 = random.gen();
        *pixel = image::Rgb([r, g, b]);
    }
    img.save("image.png").unwrap();
}