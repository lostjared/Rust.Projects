use mxr::mxr::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String> {
    let mut mx = MXWindowBuilder::new()
        .create("Hello World", 1440, 1080)
        .build()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("./data/font.ttf", 32)?;
    let tc = mx.can.texture_creator();
    let tex = mx.printtext_texture(
        &font,
        &tc,
        sdl2::pixels::Color::RGB(255, 0, 0),
        "Generated at start up",
    )?;
    let tex_w = tex_get_size(&tex);
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
        mx.printtext(
            &font,
            15,
            15,
            sdl2::pixels::Color::RGB(255, 255, 255),
            &format!("Hello, World! {} frames drawn", frame_counter),
        )?;
        mx.can.copy(
            &tex,
            None,
            sdl2::rect::Rect::new(100, 100, tex_w.0, tex_w.1),
        )?;
        mx.can.present();
        frame_counter += 1;
    }
    Ok(())
}
