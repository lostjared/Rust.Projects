use mxr::mxr::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::io::Result;

fn main() -> Result<()> {
    let mut mx = WindowBuilder::new().create("Hello World", 1440, 1080).build();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let font = ttf_context.load_font("./data/font.ttf", 18).expect("test");
    let surf = sdl2::surface::Surface::load_bmp("./data/logo.bmp").expect("on load bitmap");
    let tc = mx.can.texture_creator();
    let logo_tex = tc.create_texture_from_surface(surf).expect("on load surface");
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
        mx.can.copy(&logo_tex, None, None).expect("on err");
        mx.printtext(
            &font,
            15,
            15,
            sdl2::pixels::Color::RGB(255, 255, 255),
            &format!("Hello, World! {} frames drawn", frame_counter),
        );
        mx.can.present();
        frame_counter += 1;
    }
    Ok(())
}
