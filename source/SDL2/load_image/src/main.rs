// cargo run --release

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::path::Path;
use sdl2::surface::Surface;

fn main() {
    let pathval = Path::new("./image.bmp");
    let width = 1280;
    let height = 720;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("App", width, height).resizable().opengl().build().unwrap();
    let mut can = window.into_canvas().build().map_err(|e| e.to_string()).expect("Error on canvas");
    let tc = can.texture_creator();
    let surf = Surface::load_bmp(pathval).unwrap();
    let texture = tc.create_texture_from_surface(surf).unwrap();
    let mut e = sdl.event_pump().unwrap();
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
        can.clear();
        can.copy(&texture, None, Some(Rect::new(0, 0, width, height))).expect("on copy");
        can.present();
    }
}