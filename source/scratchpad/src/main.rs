use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const SIZE_H: usize = 720;
const SIZE_W: usize = 1280;
const SIZE_X: usize = 8;

fn getpos(x: i32, y: i32) -> Option<(usize, usize)> {
    for i in 0..SIZE_W / SIZE_X {
        for z in 0..SIZE_H / SIZE_X {
            if x as usize >= i * SIZE_X
                && x as usize <= i * SIZE_X + SIZE_X
                && y as usize >= z * SIZE_X
                && y as usize <= z * SIZE_X + SIZE_X
            {
                return Some((i, z));
            }
        }
    }
    None
}

fn main() {
    let width = SIZE_W;
    let height = SIZE_H;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let background = vec![Color::RGB(0, 0, 0), Color::RGB(255, 255, 255)];
    let foreground = vec![
        Color::RGB(255, 0, 0),
        Color::RGB(0, 255, 0),
        Color::RGB(0, 0, 255),
        Color::RGB(255, 255, 0),
        Color::RGB(0, 255, 255),
        Color::RGB(255, 0, 255),
    ];

    let mut index: usize = 0;
    let mut fore_index: usize = 0;
    let mut pixels: Box<[[u8; SIZE_H / SIZE_X]; SIZE_W / SIZE_X]> =
        Box::new([[0; SIZE_H / SIZE_X]; SIZE_W / SIZE_X]);
    let window = video
        .window(
            "Scrachpad - [Press Space to Clear]",
            width as u32,
            height as u32,
        )
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
                    for i in 0..SIZE_W / SIZE_X {
                        for z in 0..SIZE_H / SIZE_X {
                            pixels[i][z] = 0;
                        }
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if index < background.len() - 1 {
                        index += 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if index > 0 {
                        index -= 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if fore_index < foreground.len() - 1 {
                        fore_index += 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if fore_index > 0 {
                        fore_index -= 1;
                    }
                }
                _ => {}
            }
        }
        can.set_draw_color(Color::RGB(0, 0, 0));
        can.clear();
        let mut color = &background[index];
        can.set_draw_color(*color);
        can.fill_rect(Some(Rect::new(0, 0, width as u32, height as u32)))
            .expect("on fill");
        for i in 0..SIZE_W / SIZE_X {
            for z in 0..SIZE_H / SIZE_X {
                let pos: &u8 = &pixels[i][z];
                can.set_draw_color(Color::RGB(0, 0, 0));
                if *pos != 0 {
                    let x = i as i32;
                    let y = z as i32;
                    color = &foreground[fore_index];
                    can.set_draw_color(*color);
                    can.fill_rect(Some(Rect::new(
                        x * SIZE_X as i32,
                        y * SIZE_X as i32,
                        SIZE_X as u32,
                        SIZE_X as u32,
                    )))
                    .expect("on fill");
                }
            }
        }
        can.present();
        if e.mouse_state().left() {
            let pos = getpos(e.mouse_state().x(), e.mouse_state().y());
            if pos != None {
                let p = pos.unwrap();
                pixels[p.0][p.1] = 1;
            }
        }
    }
}
