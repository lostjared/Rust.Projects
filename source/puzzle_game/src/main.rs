
// cargo run --release

mod puzzle;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use puzzle::game;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

fn draw_grid(grid : &game::Grid, colors: &Vec<Color>, can: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let offset = 0;
    for x in 0..grid.get_width() as usize {
        for y in 0..grid.get_height() as usize {
            let color = grid.get_grid_point(x, y);
            if color >= 1 {
                let value: Color = *colors.get(color as usize).unwrap();
                can.set_draw_color(value);
                can.fill_rect(Some(Rect::new(x as i32 * 32, (y as i32 * 16) + offset, 32, 16))).expect("draw rect");
            } else if color < 0 {
                let mut rng = rand::thread_rng();
                let value : Color = Color::RGB(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255));
                can.set_draw_color(value);
                can.fill_rect(Some(Rect::new(x as i32 * 32, (y as i32 * 16) + offset, 32, 16))).expect("draw rect");
            } else if color == 0 {
                can.set_draw_color(Color::RGB(0, 0, 0));
                can.fill_rect(Some(Rect::new((x as i32 * 32)+1, (y as i32 * 16) + offset + 1, 31, 15))).expect("draw rect");
            }
        }
    }
    let block = grid.get_block();
    let mut value: Color = *colors.get(block[0].color as usize).unwrap();
    can.set_draw_color(value);
    can.fill_rect(Some(Rect::new(block[0].x as i32 * 32, (block[0].y as i32 * 16) + offset, 32, 16))).expect("draw rect");
    value = *colors.get(block[1].color as usize).unwrap();
    can.set_draw_color(value);
    can.fill_rect(Some(Rect::new(block[1].x as i32 * 32, (block[1].y as i32 * 16) + offset, 32, 16))).expect("draw rect");
    value = *colors.get(block[2].color as usize).unwrap();
    can.set_draw_color(value);
    can.fill_rect(Some(Rect::new(block[2].x as i32 * 32, (block[2].y as i32 * 16) + offset, 32, 16))).expect("draw rect");
}

fn main() {   

    let width = game::WINDOW_WIDTH as u32;
    let height = game::WINDOW_HEIGHT as u32;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let timer_delay : u64 = 1500;

    let window = video.window("Generic Puzzle Game", width, height).resizable().opengl().build().unwrap();
    let mut can = window.into_canvas().build().map_err(|e| e.to_string()).expect("Error on canvas");
    let tc = can.texture_creator();
    let surf = Surface::load_bmp("./img/bg.bmp").unwrap();
    let texture = tc.create_texture_from_surface(surf).unwrap();
    let mut e = sdl.event_pump().unwrap();
    can.set_draw_color(Color::RGB(0, 0, 0));
    can.clear();
    can.present();
    let mut grid : game::Grid = game::Grid::new(1280/32, 720/16);
    grid.new_piece();
    let mut colors = vec![];
    colors.push(Color::RGB(0, 0, 0));
    colors.push(Color::RGB(255, 0, 0));
    colors.push(Color::RGB(0, 255, 0));
    colors.push(Color::RGB(0, 0, 255));
    colors.push(Color::RGB(255, 255, 0));
    colors.push(Color::RGB(0, 255, 255));
    colors.push(Color::RGB(255, 255, 255));
    colors.push(Color::RGB(255, 0, 255,));
    colors.push(Color::RGB(150, 0, 40));
    colors.push(Color::RGB(50, 155, 255));

    let mut prev_tick : u64 = 0;
    let mut tick_count : u64 = 0;
    'main: loop {
        let start = SystemTime::now();
        let se = start.duration_since(UNIX_EPOCH).expect("error on time");
        let tick = se.as_secs() * 1000 + se.subsec_nanos() as u64 / 1_000_000;
        let ptick = tick - prev_tick;
        prev_tick = tick;
        tick_count += ptick;
        if tick_count > timer_delay {
            tick_count = 0;
            grid.move_down();
         }
         grid.proc_blocks();
         
        for _event in e.poll_iter() {
            match _event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                | Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => grid.move_left(),
                | Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => grid.move_right(),
                | Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } 
                | Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => grid.swap_piece_colors(0),
                | Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => grid.swap_piece_colors(1),
                | Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => grid.move_down(),
                |
                _ => {}
            }
        }
        can.set_draw_color(Color::RGB(0, 0, 0));
        can.clear();
        can.copy(&texture, None, Some(Rect::new(0, 0, width, height))).expect("on copy");
        draw_grid(&grid,&colors,&mut can);
        //   can.copy(&texture, None, Some(Rect::new(0, 0, width, height))).expect("on copy");
        can.present();
    }
}