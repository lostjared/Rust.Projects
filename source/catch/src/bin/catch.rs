use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use std::time::{SystemTime, UNIX_EPOCH};

use catch::game::Game;
use catch::game::Movement;

/// main function - entry point
fn main() {
    let mut game: Game = Game::new(1280i32, 720i32);
    game.new_game();
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

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let tc = can.texture_creator();
    let font = ttf_context
        .load_font("./font.ttf", 18)
        .expect("error loading font");
    let _text_surf = font
        .render("Hello, World!")
        .blended(Color::RGB(255, 255, 255))
        .unwrap();

    let mut e = sdl.event_pump().unwrap();
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
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    game.keypress(Movement::Left);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    game.keypress(Movement::Right);
                }
                _ => {}
            }
        }

        can.set_draw_color(Color::RGB(0, 0, 0));
        can.clear();
        game.draw(&mut can);

        let turn_surf = font
            .render(&format!("{}", game.menu_string()))
            .blended(Color::RGB(255, 255, 255))
            .unwrap();
        let turn_surf_text = tc.create_texture_from_surface(&turn_surf).unwrap();

        let TextureQuery {
            width: wi,
            height: hi,
            ..
        } = turn_surf_text.query();

        can.copy(&turn_surf_text, None, Some(Rect::new(25, 25, wi, hi)))
            .expect("on copy");

        can.present();

        let start = SystemTime::now();
        let se = start.duration_since(UNIX_EPOCH).expect("error on time");
        let tick = se.as_secs() * 1000 + se.subsec_nanos() as u64 / 1_000_000;
        let ptick = tick - prev_tick;
        prev_tick = tick;
        tick_count += ptick;
        if tick_count > 75 {
            tick_count = 0;
            game.logic();
        }
    }
}
