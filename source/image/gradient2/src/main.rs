extern crate image;

fn main() {

  let width = 1920;
  let height = 1080;
  let mut img = image::ImageBuffer::new(width, height);
  for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r : u8 = (0.3 * x as f32) as u8;
        let g : u8 = 0;
        let b : u8 = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, g, b]);
    }
    img.save("image.png").unwrap();
}