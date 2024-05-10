use mxr::mxr::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String> {
    let mut mx = MXWindowBuilder::new()
        .create("Hello World [load_gfx]", 1440, 1080)
        .build()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("./data/font.ttf", 18)?;
    let tc = mx.can.texture_creator();
    let files = vec!["./data/logo.bmp"];
    let textures = mx.load_gfx(files, &tc, None)?;
    let tex = mx
        .printtext_texture(
            &font,
            &tc,
            sdl2::pixels::Color::RGB(255, 255, 255),
            &format!("print text to texture test"),
        )
        .unwrap();

    let tex_s = tex_get_size(&tex);

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
        mx.can
            .copy(&tex, None, sdl2::rect::Rect::new(15, 15, tex_s.0, tex_s.1))
            .expect("on copy");
        mx.can.present();
    }
    Ok(())
}
