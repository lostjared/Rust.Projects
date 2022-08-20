use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{SystemTime, UNIX_EPOCH};

use lscript::scr::Direction;
use lscript::scr::MovementObject;

/// main function - entry point
fn main() {
    let args: Vec<String> = std::env::args().collect();
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

    let mut movement = MovementObject::load_from_file(args.get(1).unwrap());
    movement.print_movement();

    let mut cur_pos = ((1280 / 32) / 2, (720 / 32) / 2);

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
            match m.direction {
                Direction::Left => cur_pos.0 -= m.steps,
                Direction::Right => cur_pos.0 += m.steps,
                Direction::Up => cur_pos.1 -= m.steps,
                Direction::Down => cur_pos.1 += m.steps,
                Direction::Set => cur_pos = m.pos,
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
        }
    }
}
