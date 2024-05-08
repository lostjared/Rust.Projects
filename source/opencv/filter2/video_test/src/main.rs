use opencv::{
    core::{self, Vec3b},
    highgui,
    prelude::*,
    videoio,
};


use opencv::core::CV_8UC3;
use opencv::core::CV_32FC3;

fn proc_image(image: &mut Mat, scale: &mut f64) -> opencv::Result<()> {
    let rows = image.rows();
    let cols = image.cols();
       for z in 0..rows {
        let mut row = image.row_mut(z)?;
        for i in 0..cols {
            let mut pixel = *row.at_mut::<Vec3b>(i)?;
            pixel[0] = ((pixel[0] as f64 * *scale) % 256.0) as u8;
            pixel[1] = ((pixel[1] as f64 * *scale) % 256.0) as u8;
            pixel[2] = ((pixel[2] as f64 * *scale) % 256.0) as u8;
            *row.at_mut::<Vec3b>(i)? = pixel;
        }
    }

    *scale += 0.05;
    if *scale > 2.0 {
        *scale = 1.0;
    }

    Ok(())
}



fn main() -> opencv::Result<()> {
    let window = "video_test";
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Requires one argument: Video file");
    }
    let filename = &args[1];
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    let mut cam = videoio::VideoCapture::from_file(filename, videoio::CAP_ANY)?;
    if !videoio::VideoCapture::is_opened(&cam)? {
        panic!("Unable to open video file");
    }
    let mut scale: f64 = 1.0;
    loop {
        let mut frame = core::Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            proc_image(&mut frame, &mut scale)?;
            highgui::imshow(window, &frame)?;
        } else {
            break;
        }
        if highgui::wait_key(1)? > 0 {
            break;
        }
    }
    Ok(())
}
