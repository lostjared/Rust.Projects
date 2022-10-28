
// cargo run --release

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use rand::Rng;


const LETTER_MAX: usize = 16;
const LETTER_NUM: usize = 16;

#[derive(Copy,Clone,Debug)]
struct Letter {
    ch: char,
    xpos: i32,
    ypos: i32,
}

struct LetterGen {
    letters: Vec<Box<[Letter; LETTER_MAX]>>
}

impl LetterGen {

    fn new() -> Self {
        let mut x = 0;
        let mut v = Vec::new();
        for i in 0..LETTER_NUM {
            let  mut l = Box::new([Letter { ch: '0', xpos: x, ypos: 0}; LETTER_MAX]);
            let mut y = 0;
            for z in 0..LETTER_MAX {
                l[z].ch = 'X';
                l[z].xpos = x;
                l[z].ypos = y;
                y += 16;
            }
            v.push(l);
            x += 16;
        }

        LetterGen {
            letters: v,
        }

    }

}

fn main() {
    let width = 1280;
    let height = 720;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("App", width, height).resizable().opengl().build().unwrap();
    let mut can = window.into_canvas().build().map_err(|e| e.to_string()).expect("Error on canvas");
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let tc = can.texture_creator();
    let font = ttf_context
        .load_font("./font.ttf", 24)
        .expect("error loading font");
    let _text_surf = font
        .render("Hello, World!")
        .blended(sdl2::pixels::Color::RGB(255, 255, 255))
        .unwrap();
    let mut e = sdl.event_pump().unwrap();
    let mut rng = rand::thread_rng();
    let mut letters = LetterGen::new();

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
        can.present();
    }
}
