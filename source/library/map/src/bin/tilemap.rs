// cargo run --bin tilemap
// followed this tutorial here: https://developer.mozilla.org/en-US/docs/Games/Techniques/Tilemaps
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::Path;
use sdl2::surface::Surface;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashSet;
use map::level::Camera;
use map::level::Map;

fn fmin(x: f64, x2: f64) -> f64 {
    if x < x2 {
        x
    } else {
        x2
    }
}

fn main() {
    let pathval = Path::new("./image.bmp");
    let width = 512;
    let height = 512;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("Tilemap App", width, height).resizable().opengl().build().unwrap();
    let mut can = window.into_canvas().build().map_err(|e| e.to_string()).expect("Error on canvas");
    let tc = can.texture_creator();
    let surf = Surface::load_bmp(pathval).unwrap();
    let texture = tc.create_texture_from_surface(surf).unwrap();
    let mut e = sdl.event_pump().unwrap();
    let tiles = vec![
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3,
        3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3,
        3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3,
        3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3,
        3, 1, 1, 1, 1, 1, 2, 1, 1, 1, 3, 3,
        3, 1, 2, 2, 1, 1, 1, 1, 1, 1, 3, 3,
        3, 1, 2, 2, 1, 1, 1, 1, 1, 1, 3, 3,
        3, 1, 1, 1, 2, 1, 1, 1, 1, 1, 3, 3,
        3, 1, 1, 1, 1, 2, 1, 1, 1, 1, 3, 3,
        3, 1, 1, 1, 1, 2, 1, 1, 1, 1, 3, 3,
        3, 3, 3, 1, 1, 2, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 1, 1, 2, 3, 3, 3, 3, 3, 3,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];
    let map = Map::new(12, 12, 64, tiles);
    let max_x : i32 = (map.cols * 64 - 512)-1;
    let max_y : i32 = (map.cols * 64 - 512)-1;
    let mut camera = Camera::new(512, 512, max_x, max_y);
    let mut prev_tick : u64 = 0;
    'main: loop {
        let start = SystemTime::now();
        let se = start.duration_since(UNIX_EPOCH).expect("error on time");
        let tick = se.as_secs() * 1000 + se.subsec_nanos() as u64 / 1_000_000;
        let mut delta : f64 = (tick as f64 - prev_tick as f64) / 1000.0;
        prev_tick = tick;
        delta = fmin(0.75, delta);
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
        map.draw_map(&texture, &camera, &mut can);
        can.present();
        let keys : HashSet<_> = e.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        let mut move_x : i32 = 0;
        let mut move_y : i32 = 0;
        for i in &keys {
            match *i {
                Keycode::Left => {
                    move_x = -1;
                }
                Keycode::Right => {
                     move_x = 1;
                }
                Keycode::Up => {
                    move_y = -1;
                }
                Keycode::Down => {
                    move_y = 1;
                }
                    _ => {}
                }
            }
            if move_x != 0 || move_y != 0 {
                camera.move_camera(delta, move_x, move_y);
                println!("x: {} y: {} delta: {}", move_x, move_y, delta);
            }
   }
}