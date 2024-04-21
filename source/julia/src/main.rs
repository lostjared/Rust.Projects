use rand::Rng;

pub fn parse_split_int(t: &str) -> (i32, i32) {
    let text = String::from(t.trim());
    let pos = text.find('x');
    if pos == None {
        panic!("Could not find list seperator for argument!..");
    }
    let pos_value = pos.unwrap();
    let left = &text[0..pos_value];
    let right = &text[pos_value + 1..text.len()];
    (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
}

pub fn parse_split_double(t: &str) -> (f32, f32) {
    let text = String::from(t.trim());
    let pos = text.find(',');
    if pos == None {
        panic!("Could not find list seperator for argument..");
    }
    let pos_value = pos.unwrap();
    let left = &text[0..pos_value];
    let right = &text[pos_value + 1..text.len()];
    (left.parse::<f32>().unwrap(), right.parse::<f32>().unwrap())
}

fn main() -> std::io::Result<()> {
    println!("output.png 800x600 paramA,paramB iter");
    let args : Vec<String> = std::env::args().collect();
    let res = parse_split_int(&args[2]);
    let param = parse_split_double(&args[3]);
    let iter = args[4].parse::<i32>();
    draw_julia(&args[1], res, param, iter.unwrap());
    Ok(())
}

pub fn draw_julia(filename: &str, res: (i32, i32), param: (f32, f32), iter: i32) {
    let imgx = res.0 as u32;
    let imgy = res.1 as u32;
    let mut rng = rand::thread_rng();
    let mut color_map : Vec<(u8, u8, u8)> = Vec::new();
    for _i in 0..=iter {
        let r = rng.gen::<u8>(); 
        let g = rng.gen::<u8>(); 
        let b = rng.gen::<u8>(); 
        color_map.push((r, g, b));
    }


    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * y as f32) as u8;
        let g = (0.3 * x as f32) as u8;
        *pixel = image::Rgb([r, g, 0]);
    }
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;
            let c = num::complex::Complex::new(param.0, param.1);
            let mut z = num::complex::Complex::new(cx, cy);
            let mut i = 0;
            while i < iter && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }
            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(_data) = *pixel;
            let color = color_map[i as usize];
            *pixel = image::Rgb([color.0, color.1, color.2]);
        }
    }
    imgbuf.save(&filename).unwrap();
}
