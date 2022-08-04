use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
fn main() {
    let width = 1280;
    let height = 720;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("App", width, height)
        .resizable()
        .opengl()
        .build()
        .unwrap();
    let mut can = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .expect("Error on canvas");
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
        can.set_draw_color(Color::RGB(0, 0, 0));
        can.clear();

        can.set_draw_color(Color::RGB(255, 255, 255));
        can.fill_rect(Some(Rect::new(100, 200, 1280 - 200, 25)))
            .expect("draw rect");
        can.fill_rect(Some(Rect::new(100, 400, 1280 - 200, 25)))
            .expect("draw rect");
        can.fill_rect(Some(Rect::new(1280 / 2 - 200, 25, 25, 720 - 100)))
            .expect("draw rect");
        can.fill_rect(Some(Rect::new(1280 / 2 + 200, 25, 25, 720 - 100)))
            .expect("draw rect");
        can.present();
    }
}
