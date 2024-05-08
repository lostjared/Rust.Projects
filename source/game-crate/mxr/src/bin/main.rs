use mxr::mxr::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> std::io::Result<()> {
    let mut mx = WindowBuilder::new().create("Hello World", 640, 480).build();
    let mut e = mx.sdl.event_pump().unwrap();
    'main: loop {
        for _event in e.poll_iter() {
            match _event {
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
