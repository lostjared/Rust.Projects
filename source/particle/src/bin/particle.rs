use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{SystemTime, UNIX_EPOCH};

use particle::particle_emiter::Emiter;
use particle::particle_emiter::NUM_PARTICLES;

/// main function - entry point
fn main() {
    let width = 1280;
    let height = 720;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("Particle", width, height)
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
    let mut emiter = Emiter::new();
    emiter.init();
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

        for i in 0..NUM_PARTICLES {
            let p = emiter[i];
            can.set_draw_color(Color::RGB(p.depth, p.depth, p.depth));
            can.fill_rect(Some(Rect::new(p.x, p.y, 4, 4)))
                .expect("on fill");
        }

        can.present();
        let start = SystemTime::now();
        let se = start.duration_since(UNIX_EPOCH).expect("error on time");
        let tick = se.as_secs() * 1000 + se.subsec_nanos() as u64 / 1_000_000;
        let ptick = tick - prev_tick;
        prev_tick = tick;
        tick_count += ptick;
        if tick_count > 75 {
            tick_count = 0;
            emiter.update();
        }
    }
}
