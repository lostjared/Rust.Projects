use opencv::{
    core::{self, Vec3b},
    highgui,
    prelude::*,
    videoio,
};

fn proc_image(image: &mut Mat, scale: &mut f32) -> opencv::Result<()> {
    for z in 0..image.rows() {
        for i in 0..image.cols() {
            let pixel = image.at_2d_mut::<Vec3b>(z, i)?;
            pixel[0] = ((pixel[0] as f32 * *scale) as u32 % 256) as u8;
            pixel[1] = ((pixel[1] as f32 * *scale) as u32 % 256) as u8;
            pixel[2] = ((pixel[2] as f32 * *scale) as u32 % 256) as u8;
        }
    }
    *scale += 0.05;
    if *scale > 6.0 {
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
    let mut scale: f32 = 1.0;
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
