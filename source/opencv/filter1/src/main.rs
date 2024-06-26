use opencv::{
    core::{Mat, Vec3b},
    highgui, imgcodecs,
    prelude::*,
    Result,
};

fn proc_image(image: &mut Mat, scale: &mut f32) -> Result<(), Box<dyn std::error::Error>> {
    let rows = image.rows();
    let cols = image.cols();
    for z in 0..rows {
        let mut row = image.row_mut(z)?;
        for i in 0..cols {
            let mut pixel = *row.at_mut::<Vec3b>(i)?;
            pixel[0] = ((pixel[0] as f32 * *scale) % 256.0) as u8;
            pixel[1] = ((pixel[1] as f32 * *scale) % 256.0) as u8;
            pixel[2] = ((pixel[2] as f32 * *scale) % 256.0) as u8;
            *row.at_mut::<Vec3b>(i)? = pixel;
        }
    }

    *scale += 0.05;
    if *scale > 2.0 {
        *scale = 1.0;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argz: Vec<String> = std::env::args().collect();
    if argz.len() != 2 {
        println!("Error requires one argument file to load");
        std::process::exit(1);
    }
    let window = "Self Scale";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    let file_name = &argz[1];
    let img = imgcodecs::imread(file_name, imgcodecs::IMREAD_COLOR)?;
    if img.empty() {
        println!("Could not open or find the image");
        std::process::exit(1);
    }
    let mut scale: f32 = 1.0;
    loop {
        let mut cur_image = img.clone();
        proc_image(&mut cur_image, &mut scale)?;
        highgui::imshow(window, &cur_image)?;
        let k = highgui::wait_key(1);
        if let Ok(key) = k {
            if key == 27 || key == 'q' as i32 {
                break;
            }
        }
    }
    Ok(())
}
