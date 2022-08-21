use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{SystemTime, UNIX_EPOCH};

use lscript::scr::Direction;
use lscript::scr::MovementObject;

const TILE_W: usize = 1280 / 32;
const TILE_H: usize = 720 / 32;

/// screen pixels structure
#[derive(Copy, Clone)]
pub struct Pixel {
    on: bool,
    color: (u8, u8, u8),
}

/// pixel grid
pub struct PixelGrid {
    pub pixels: Box<[[Pixel; TILE_H]; TILE_W]>,
}

impl PixelGrid {
    pub fn new() -> Self {
        PixelGrid {
            pixels: Box::new(
                [[Pixel {
                    on: false,
                    color: (0, 0, 0),
                }; TILE_H]; TILE_W],
            ),
        }
    }

    pub fn clear(&mut self) {
        for i in 0..TILE_W {
            for z in 0..TILE_H {
                self.pixels[i][z].on = false;
                self.pixels[i][z].color = (0, 0, 0);
            }
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, col: (u8, u8, u8)) {
        self.pixels[x][y].on = true;
        self.pixels[x][y].color = col;
    }
}

/// main function - entry point
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut movement = MovementObject::load_from_file(args.get(1).unwrap());
    movement.print_movement();

    let mut grid : PixelGrid = PixelGrid::new();
    
    let width = 1280;
    let height = 720;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("LScript", width, height)
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
    let mut prev_tick: u64 = 0;
    let mut tick_count = 0;



    
    let mut cur_pos = ((1280 / 32) / 2, (720 / 32) / 2);
    let mut cur_color = (255,255,255);

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
        for i in 0..TILE_W {
            for z in 0..TILE_H {
                let pix = grid.pixels[i][z];
                if pix.on  {
                can.set_draw_color(Color::RGB(pix.color.0, pix.color.1, pix.color.2));
                    can.fill_rect(Some(Rect::new(i as i32 * 32, z as i32 * 32, 32, 32))).expect("on rect");
                }
            }
        }
        can.set_draw_color(Color::RGB(cur_color.0, cur_color.1, cur_color.2));
        can.fill_rect(Some(Rect::new(cur_pos.0 * 32, cur_pos.1 * 32, 32, 32)))
            .expect("on rect");
    
            can.present();
        let start = SystemTime::now();
        let se = start.duration_since(UNIX_EPOCH).expect("error on time");
        let tick = se.as_secs() * 1000 + se.subsec_nanos() as u64 / 1_000_000;
        let ptick = tick - prev_tick;
        prev_tick = tick;
        tick_count += ptick;
        if tick_count > 75 {
            tick_count = 0;
            let m = movement.get_pos();
            println!("{}", m);
            match m.direction {
                Direction::Left => cur_pos.0 -= m.steps,
                Direction::Right => cur_pos.0 += m.steps,
                Direction::Up => cur_pos.1 -= m.steps,
                Direction::Down => cur_pos.1 += m.steps,
                Direction::Set => cur_pos = m.pos,
                Direction::Color => {
                    match m.steps {
                        0 => cur_color = (255, 0, 0),
                        1 => cur_color = (0, 255, 0),
                        2 => cur_color = (0, 0, 255),
                        _ => cur_color = (255,255,255),
                    }
                }
            }
            // bounds check
            if cur_pos.0 < 0 {
                cur_pos.0 = 0;
            }
            if cur_pos.1 < 0 {
                cur_pos.1 = 0;
            }
            if cur_pos.0 > (1280 / 32) - 1 {
                cur_pos.0 = (1280 / 32) - 1;
            }
            if cur_pos.1 > (720 / 32) - 1 {
                cur_pos.1 = (720 / 32) - 1;
            }

            grid.set_pixel(cur_pos.0 as usize, cur_pos.1 as usize,cur_color);

            if movement.index == 0 {
                grid.clear();
            }
        }
    }
}
