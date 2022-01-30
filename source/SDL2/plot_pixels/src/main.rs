// cargo run --release

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::path::Path;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
fn main() {
    let pathval = Path::new("./image.bmp");
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let width = 640;
    let height = 360;
    let window = video.window("App", width, height).resizable().opengl().build().unwrap();
    let mut can = window.into_canvas().build().map_err(|e| e.to_string()).expect("Error on canvas");
    let tc = can.texture_creator();
    let mut e = sdl.event_pump().unwrap();
    let surf = Surface::load_bmp(pathval).unwrap();
    let texture_surf = tc.create_texture_from_surface(surf).unwrap();
    let mut texture = tc.create_texture_streaming(PixelFormatEnum::RGB24, width, height).map_err(|e| e.to_string()).expect("Error on texture create");
    let mut xval : f32 = 1.0;
    can.clear();
    can.copy(&texture_surf, None, Some(Rect::new(0, 0, width, height))).expect("on copy");
    let pixels = can.read_pixels(can.viewport(), PixelFormatEnum::RGB24).unwrap();
    can.present();
    let mut dir = 1;
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
            if dir == 1 {
                xval += 0.01;
                if xval > 5.0 {
                    dir = 0;
                }
            } else {
                xval -= 0.01;
                if xval <= 1.0 {
                    dir = 1;
                }
            }
            for y in 0..height {
                for x in 0..width {
                    let offset = y  * pitch as u32 + x * 3;
                    let r = pixels[offset as usize] as f32 * xval;
                    let g = pixels[offset as usize + 1] as f32 * xval;
                    let b = pixels[offset as usize + 2] as f32 * xval;
                    let mut ri = r as u32;
                    let mut gi = g as u32;
                    let mut bi = b as u32;
                    if ri > 255 {
                        ri %= 255;
                    }
                    if gi > 255 {
                        gi %= 255;
                    }
                    if bi > 255 {
                        bi %= 255;
                    }
                    buffer[offset as usize] = ri as u8;
                    buffer[offset as usize + 1] = gi as u8;
                    buffer[offset as usize + 2] = bi as u8;
                }
            }
        }).expect("on lock"); 
        can.clear();
        can.copy(&texture, None, Some(Rect::new(0, 0, width, height))).expect("on copy");
        can.present();
    }
}