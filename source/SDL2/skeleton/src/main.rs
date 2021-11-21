
// cargo run --release

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use rand::Rng;

fn main() {
    let width = 1280;
    let height = 720;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("App", width, height).resizable().opengl().build().unwrap();
    let mut can = window.into_canvas().build().map_err(|e| e.to_string()).expect("Error on canvas");
    let tc = can.texture_creator();
    let mut texture = tc.create_texture_streaming(PixelFormatEnum::RGB24, width, height).map_err(|e| e.to_string()).expect("Error on texture create");
    let mut e = sdl.event_pump().unwrap();
    let mut rng = rand::thread_rng();
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
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
           for y in 0..height {
                for x in 0..width {
                    let offset = y  * pitch as u32 + x * 3;
                    buffer[offset as usize] = rng.gen_range(0..255) as u8;
                    buffer[offset as usize + 1] = rng.gen_range(0..255) as u8;
                    buffer[offset as usize + 2] = rng.gen_range(0..255) as u8;
                }
            }
        }).expect("on lock");
        can.clear();
        can.copy(&texture, None, Some(Rect::new(0, 0, width, height))).expect("on copy");
        can.present();
    }

}
