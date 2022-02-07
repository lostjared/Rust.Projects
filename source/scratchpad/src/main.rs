use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

static WIDTH: u32 = 1280;
static HEIGHT: u32 = 720;

fn getpos(x: i32, y: i32) -> (usize, usize) {
    for i in 0..1280 / 16_usize {
        for z in 0..720 / 16_usize {
            if x as usize >= i * 16
                && x as usize <= i * 16 + 16
                && y as usize >= z * 16
                && y as usize <= z * 16 + 16
            {
                return (i, z);
            }
        }
    }
    (0, 0)
}

fn main() {
    let width = WIDTH;
    let height = HEIGHT;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let mut pixels: Box<[[u8; 720 / 16]; 1280 / 16]> = Box::new([[0; 720 / 16]; 1280 / 16]);

    let window = video
        .window("Scrachpad - [Press Space to Clear]", width, height)
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
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    for i in 0..1280 / 16_usize {
                        for z in 0..720 / 16_usize {
                            pixels[i][z] = 0;
                        }
                    }
                }
                _ => {}
            }
        }
        can.set_draw_color(Color::RGB(0, 0, 0));
        can.clear();

        for i in 0..1280 / 16_usize {
            for z in 0..720 / 16_usize {
                let pos: &u8 = &pixels[i][z];
                can.set_draw_color(Color::RGB(0, 0, 0));
                if *pos != 0 {
                    let x = i as i32;
                    let y = z as i32;
                    can.set_draw_color(Color::RGB(255, 255, 255));
                    can.fill_rect(Some(Rect::new(x * 16_i32, y * 16_i32, 16, 16)))
                        .expect("on fill");
                }
            }
        }
        can.present();

        if e.mouse_state().left() {
            let pos = getpos(e.mouse_state().x(), e.mouse_state().y());
            pixels[pos.0][pos.1] = 1;
        }
    }
}
