
use res_ex::res_ex::extract_resolution;
// cargo run --release --  1280x720
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

use rand::Rng;

fn main() {

    let args : Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Error reqiores one argument use format WidthxHeight");
        std::process::exit(-1);
    }

    let res = extract_resolution(&args[1]);    
    let width = res.width;
    let height = res.height;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("Resize", width, height).resizable().opengl().build().unwrap();
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