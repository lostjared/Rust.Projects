use mxr::mxr::*;

// cargo run --release

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

struct Screen1 {}

impl Screen for Screen1 {
    fn draw(&mut self, can: &mut sdl2::render::Canvas<sdl2::video::Window>, w: u32, h: u32) {
        can.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        can.fill_rect(Some(Rect::new(0, 0, w, h)))
            .expect("on drawing of rect");
    }
    fn event(&mut self, e: &sdl2::event::Event) {
        match e {
            Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => {
                println!("enter");
            }
            _ => {}
        }
    }
}

fn main() -> std::io::Result<()> {
    let width = 1280;
    let height = 720;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("Screens", width, height)
        .resizable()
        .opengl()
        .build()
        .unwrap();
    let mut can = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .expect("Error on canvas");
    let _tc = can.texture_creator();
    //    let mut texture = tc.create_texture_streaming(PixelFormatEnum::RGB24, width, height).map_err(|e| e.to_string()).expect("Error on texture create");
    let mut scr_obj = ScreenObjects::new(width, height);
    scr_obj.push_screen(Box::new(Screen1 {}));
    scr_obj.set_screen(0);

    let mut e = sdl.event_pump().unwrap();
    'main: loop {
        for _event in e.poll_iter() {
            match _event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {
                    scr_obj.event(&_event);
                }
            }
        }
        can.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        can.clear();
        scr_obj.draw(&mut can);
        can.present();
    }
    Ok(())
}
