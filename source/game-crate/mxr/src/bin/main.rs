use mxr::mxr::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::io::Result;

fn main() -> Result<()> {
    let mut mx = WindowBuilder::new().create("Hello World", 640, 480).build();
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
        mx.can.present();
    }
    Ok(())
}
