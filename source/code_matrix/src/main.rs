// Looks kind of like a Matrix code emulator
// this version allows you to supply the string data to use
// cargo run --release -- --input src/main.rs
// --input text file with string data to use (required)
// --timeout timeout delay (how fast it moves)
// --color the color of the characters in format r,g,b ex: 0,255,0
// --font true type font file.
// press Up arrow to scroll up
// press Down arrow to scroll down

use clap::{App, Arg};
use logger::log::*;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureQuery;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
/// program constants
const LETTER_MAX: usize = 21;
const LETTER_NUM: usize = 40;
const LETTER_SIZE: i32 = 32;

/// on screen letter structure
#[derive(Copy, Clone, Debug)]
struct Letter {
    ch: char,
    xpos: i32,
    ypos: i32,
}

// letter generator
struct LetterGen {
    letters: Vec<[Letter; LETTER_MAX]>,
    letter_row: Vec<i32>,
}

impl LetterGen {
    /// create new letter generator
    fn new(data: &mut StringData) -> Self {
        let mut rng = rand::thread_rng();
        let mut x = 0;
        let mut v = Vec::new();
        let mut r = Vec::new();
        for _i in 0..LETTER_NUM {
            let mut l = [Letter {
                ch: '0',
                xpos: x,
                ypos: 0,
            }; LETTER_MAX];

            let mut y = -LETTER_SIZE;
            for l in l.iter_mut().take(LETTER_MAX) {
                l.ch = data.getchar();
                l.xpos = x;
                l.ypos = y;
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

struct StringData {
    data: String,
    index: usize,
    alphabet: Vec<char>,
}

impl StringData {
    fn new(input: &str) -> Self {
        if input.is_empty() {
            panic!("String is empty!");
        }
        let mut alpha: Vec<char> = Vec::new();
        for i in 'a'..='z' {
            alpha.push(i);
        }
        for i in 'A'..='Z' {
            alpha.push(i);
        }
        let symbols = String::from("~!@#$%^&*()-+=[]{}<>.,|\\/?;:`");
        for i in symbols.chars() {
            alpha.push(i);
        }
        for i in '0'..='9' {
            alpha.push(i);
        }
        let mut dat = String::new();
        for i in input.chars() {
            let ch = alpha.iter().find(|&x| *x == i);
            if ch != None {
                dat.push(i);
            }
        }
        if dat.is_empty() {
            panic!("Input string is empty");
        }
        Self {
            data: dat,
            index: 0,
            alphabet: alpha,
        }
    }
    fn getchar(&mut self) -> char {
        let ch;
        if self.index < self.data.len() {
            ch = self.data.chars().nth(self.index).unwrap();
            self.index += 1;
        } else {
            self.index = 0;
            ch = self.data.chars().nth(self.index).unwrap();
        }
        ch
    }
}

/// command line arguments structure
struct Arguments {
    color: (u8, u8, u8),
    timeout: u64,
    font: String,
    data: String,
}

/// parse a color from a string
fn parse_color(input: String) -> (u8, u8, u8) {
    let s = input.find(',');
    let sp = s.unwrap();
    let r = &input[..sp];
    let right = &input[sp + 1..];
    let gp = right.find(',');
    let gv = gp.unwrap();
    let g = &right[..gv];
    let b = &right[gv + 1..];
    (r.parse().unwrap(), g.parse().unwrap(), b.parse().unwrap())
}

/// parse command line arguments
fn parse_args() -> (Arguments, Log) {
    let m = App::new("code_matrix")
        .help("matrix code emulator")
        .author("Jared Bruni jaredbruni@protonmail.com")
        .version("0.1.0")
        .arg(
            Arg::new("color")
                .help("color of characters in format 0,0,0")
                .required(false)
                .takes_value(true)
                .default_value("0,255,0")
                .short('c')
                .long("color")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("timeout")
                .help("timeout delay")
                .required(false)
                .default_value("75")
                .short('t')
                .long("timeout")
                .allow_invalid_utf8(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("font")
                .help("font")
                .required(false)
                .short('f')
                .long("font")
                .takes_value(true)
                .default_value("font.ttf")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("input")
                .help("input file")
                .required(true)
                .short('i')
                .long("input")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("output")
                .help("log file")
                .required(false)
                .short('o')
                .long("output")
                .takes_value(true)
                .default_value("log.txt")
                .allow_invalid_utf8(true),
        )
        .get_matches();

    let log_file = m.value_of_lossy("output").unwrap();
    let mut log = Log::new_file_log("code_matrix", &log_file, false, true);

    let col = parse_color(m.value_of_lossy("color").unwrap().to_string());
    let t = m.value_of_lossy("timeout").unwrap().parse().unwrap();
    let f = m.value_of_lossy("font").unwrap();
    let input = m.value_of_lossy("input").unwrap();
    let s = std::fs::read_to_string(&input.to_string()).expect("on read to string");
    log.i(&format!("Loaded input file: {}", input));
    if s.len() < 100 {
        log.w(&format!(
            "Input file is only {} bytes should be larger",
            s.len()
        ));
    }
    (
        Arguments {
            color: col,
            timeout: t,
            font: f.to_string(),
            data: s,
        },
        log,
    )
}

/// main function
fn main() {
    let rt = parse_args();
    let args = rt.0;
    let mut log = rt.1;
    let mut data = StringData::new(&args.data);
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
        .load_font(&args.font, 32)
        .expect("error loading font");
    let _text_surf = font
        .render("Hello, World!")
        .blended(sdl2::pixels::Color::RGB(255, 255, 255))
        .unwrap();

    log.i(&format!("Font {} loaded", &args.font));
    let mut tex_map: HashMap<char, sdl2::render::Texture> = HashMap::new();

    for i in &data.alphabet {
        let ch = *i;
        let text_surf = font.render(&format!("{}", ch)).blended(color).unwrap();
        let tex = tc.create_texture_from_surface(text_surf).unwrap();
        tex_map.insert(ch, tex);
    }
    let mut e = sdl.event_pump().unwrap();
    let mut letters_st = LetterGen::new(&mut data);
    let mut prev_tick: u64 = 0;
    let mut tick_count = 0;
    let mut dir: bool = false;

    log.o("Program Loaded....");

    'main: loop {
        for _event in e.poll_iter() {
            match _event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    dir = true;
                    log.i("Direction changed to up");
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    dir = false;
                    log.i("Direction changed to down");
                }
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
            for letter in letters_st.letters[i].iter_mut().take(LETTER_MAX) {
                let ch = letter.ch;
                let x = letter.xpos;
                let speed = letters_st.letter_row[i];
                let y = &mut letter.ypos;
                let tex = tex_map.get(&ch).unwrap();
                let TextureQuery {
                    width: wi,
                    height: hi,
                    ..
                } = tex.query();
                can.copy(tex, None, Some(sdl2::rect::Rect::new(x, *y, wi, hi)))
                    .expect("on copy");
                if tick_count > args.timeout {
                    if dir {
                        *y -= speed;
                        if *y <= -LETTER_SIZE {
                            *y = 720;
                            letter.ch = data.getchar();
                        }
                    } else {
                        *y += speed;
                        if *y >= 720 {
                            *y = -LETTER_SIZE;
                            letter.ch = data.getchar();
                        }
                    }
                }
            }
        }
        if tick_count > args.timeout {
            tick_count = 0;
        }
        can.present();
    }
    log.o("Program Exited");
}
