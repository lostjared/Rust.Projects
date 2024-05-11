use mxr::mxr::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rand::Rng;
fn main() -> Result<(), String> {

    let width = 1440;
    let height = 1080;

    let mut mx = MXWindowBuilder::new()
        .create("Hello World", 1440, 1080)
        .build()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("./data/font.ttf", 18)?;
    let tc = mx.can.texture_creator();
    let mut texture = tc.create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, width, height).map_err(|e| e.to_string())?;
    let mut frame_counter: u64 = 0;
    'main: loop {
        for event in mx.event.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }
        mx.can.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        mx.can.clear();
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            let mut rng = rand::thread_rng();
            for y in 0..height {
                for x in 0..width {
                    let offset = x as usize * 3 + y as usize * pitch;
                    buffer[offset as usize] = rng.gen_range(0..256) as u8;
                    buffer[offset as usize + 1] = rng.gen_range(0..256) as u8;
                    buffer[offset as usize + 2] = rng.gen_range(0..256) as u8;
                }
            }
        })?;
        mx.can
            .copy(&texture, None, None)?;
        mx.printtext(
            &font,
            15,
            15,
            sdl2::pixels::Color::RGB(255, 255, 255),
            &format!("Hello, World! {} frames drawn", frame_counter),
        )
        .expect("Failure to print text to screen.");
        mx.can.present();
        frame_counter += 1;
    }
    Ok(())
}
