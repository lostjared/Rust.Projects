use mxr::mxr::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


fn main() -> Result<(), String> {
    let mut mx = WindowBuilder::new()
        .create("Hello World [load_gfx]", 1440, 1080)
        .build()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("./data/font.ttf", 18)?;
    let tc = mx.can.texture_creator();
    let files = vec!["./data/logo.bmp"];
    let textures = mx.load_gfx(files, &tc)?;
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
        mx.can
            .copy(&textures[0], None, None)
            .expect("Failure to copy texture to canvas");
        mx.printtext(
            &font,
            15,
            15,
            sdl2::pixels::Color::RGB(255, 255, 255),
            &format!("load_gfx() test {} frames drawn", frame_counter),
        )
        .expect("Failure to print text to screen.");
        mx.can.present();
        frame_counter += 1;
    }
    Ok(())
}
