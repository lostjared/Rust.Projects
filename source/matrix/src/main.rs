// Looks kind of like a Matrix code emulator not there 100% yet.
// cargo run --release

use clap::{App, Arg};
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureQuery;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

const LETTER_MAX: usize = 21;
const LETTER_NUM: usize = 40;
const LETTER_SIZE: i32 = 32;

#[derive(Copy, Clone, Debug)]
struct Letter {
    ch: char,
    xpos: i32,
    ypos: i32,
}

struct LetterGen {
    letters: Vec<Box<[Letter; LETTER_MAX]>>,
    letter_row: Vec<i32>,
}

impl LetterGen {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut x = 0;
        let mut v = Vec::new();
        let mut r = Vec::new();
        for _i in 0..LETTER_NUM {
            let mut l = Box::new(
                [Letter {
                    ch: '0',
                    xpos: x,
                    ypos: 0,
                }; LETTER_MAX],
            );
            let mut y = -LETTER_SIZE;
            for z in 0..LETTER_MAX {
                l[z].ch = rng.gen_range('a'..='z');
                l[z].xpos = x;
                l[z].ypos = y;
                y += LETTER_SIZE + 4;
            }
            v.push(l);
            r.push(rng.gen_range(24..32));
            x += LETTER_SIZE;
        }
        LetterGen {
            letters: v,
            letter_row: r,
        }
    }
}

struct Arguments {
    color: (u8, u8, u8),
}

fn parse_color(input: &str) -> (u8, u8, u8) {
    let s = input.find(",");
    let sp = s.unwrap();
    let r = &input[..sp];
    let right = &input[sp+1..];
    let gp = right.find(",");
    let gv = gp.unwrap();
    let g = &right[..gv];
    let b = &right[gv+1..];
    (r.parse().unwrap(),g.parse().unwrap(),b.parse().unwrap())
}

fn parse_args() -> Arguments {
    let m = App::new("matrix")
        .help("matrix")
        .arg(
            Arg::new("color")
                .required(false)
                .default_value("0,255,0")
                .short('c')
                .long("color")
                .allow_invalid_utf8(true),
        )
        .get_matches();
        let col = parse_color(&m.value_of_lossy("color").unwrap().to_string());

    Arguments { color: col }
}

fn main() {
    let args = parse_args();
    let color = sdl2::pixels::Color::RGB(args.color.0, args.color.1, args.color.2);
    let width = 1280;
    let height = 720;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("[Code Matrix]", width, height)
        .opengl()
        .build()
        .unwrap();
    let mut can = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .expect("Error on canvas");
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let tc = can.texture_creator();
    let font = ttf_context
        .load_font("./font.ttf", 32)
        .expect("error loading font");
    let _text_surf = font
        .render("Hello, World!")
        .blended(sdl2::pixels::Color::RGB(255, 255, 255))
        .unwrap();

    let mut tex_map : HashMap<char, sdl2::render::Texture> = HashMap::new();
    for i in 'a' ..= 'z' {
        let text_surf = font
        .render(&format!("{}", i))
        .blended(color)
        .unwrap();
        let tex = tc.create_texture_from_surface(text_surf).unwrap();
        tex_map.insert(i, tex);
    }

    let mut e = sdl.event_pump().unwrap();
    let mut rng = rand::thread_rng();
    let mut letters_st = LetterGen::new();
    let mut prev_tick: u64 = 0;
    let mut tick_count = 0;
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

        let start = SystemTime::now();
        let se = start.duration_since(UNIX_EPOCH).expect("error on time");
        let tick = se.as_secs() * 1000 + se.subsec_nanos() as u64 / 1_000_000;
        let ptick = tick - prev_tick;
        prev_tick = tick;
        tick_count += ptick;

        for i in 0..letters_st.letters.len() {
            for z in 0..LETTER_MAX {
                let ch = letters_st.letters[i][z].ch;
                let x = letters_st.letters[i][z].xpos;
                let speed = letters_st.letter_row[i];
                let y = &mut letters_st.letters[i][z].ypos;
                let tex = tex_map.get(&ch).unwrap();
                let TextureQuery {
                    width: wi,
                    height: hi,
                    ..
                } = tex.query();
                can.copy(&tex, None, Some(sdl2::rect::Rect::new(x, *y, wi, hi)))
                    .expect("on copy");
                if tick_count > 75 {
                    *y -= speed;
                    if *y <= -LETTER_SIZE {
                        *y = 720;
                        letters_st.letters[i][z].ch = rng.gen_range('a'..='z');
                    }
                }
            }
        }

        if tick_count > 75 {
            tick_count = 0;
        }

        can.present();
    }
}
