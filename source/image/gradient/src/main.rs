extern crate image;

fn main() {
  let mut img = image::ImageBuffer::new(256, 256);
  for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let r : u8 = 0;
        let g : u8 = 0;
        let b : u8 = 0;
        *pixel = image::Rgb([r, g, b]);
    }
    for y in 0..256 {
        for x in 0..256 {
            let pixel = img.get_pixel_mut(x, y);
            let image::Rgb(_data) = *pixel;
            *pixel = image::Rgb([x as u8,y as u8,0]);
        }
    
    }
    img.save("image.png").unwrap();
}