// cargo run --release

extern crate sdl2;
mod puzzle;

use puzzle::game;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::surface::Surface;
use std::time::{SystemTime, UNIX_EPOCH};

fn draw_grid(
    grid: &game::Grid,
    can: &mut sdl2::render::Canvas<sdl2::video::Window>,
    blocks: &Vec<sdl2::render::Texture>,
) {
    let offset = 0;
    for x in 0..grid.get_width() as usize {
        for y in 0..grid.get_height() as usize {
            let color = grid.get_grid_point(x, y);
            if color >= 1 {
                let b = blocks.get(color as usize).unwrap();
                can.copy(
                    &b,
                    None,
                    Some(Rect::new(x as i32 * 32, (y as i32 * 16) + offset, 32, 16)),
                )
                .expect("on copy block");
            } else if color < 0 {
                let mut rng = rand::thread_rng();
                let value: Color = Color::RGB(
                    rng.gen_range(0..255),
                    rng.gen_range(0..255),
                    rng.gen_range(0..255),
                );
                can.set_draw_color(value);
                can.fill_rect(Some(Rect::new(
                    x as i32 * 32,
                    (y as i32 * 16) + offset,
                    32,
                    16,
                )))
                .expect("draw rect");
            } else if color == 0 {
                can.set_draw_color(Color::RGB(0, 0, 0));
                can.fill_rect(Some(Rect::new(
                    (x as i32 * 32) + 1,
                    (y as i32 * 16) + offset + 1,
                    31,
                    15,
                )))
                .expect("draw rect");
            }
        }
    }
    let block = grid.get_block();
    let b = blocks.get(block[0].color as usize).unwrap();
    can.copy(
        &b,
        None,
        Some(Rect::new(
            block[0].x as i32 * 32,
            (block[0].y as i32 * 16) + offset,
            32,
            16,
        )),
    )
    .expect("draw rect");
    let b2 = blocks.get(block[1].color as usize).unwrap();
    can.copy(
        &b2,
        None,
        Some(Rect::new(
            block[1].x as i32 * 32,
            (block[1].y as i32 * 16) + offset,
            32,
            16,
        )),
    )
    .expect("draw rect");
    let b3 = blocks.get(block[2].color as usize).unwrap();
    can.copy(
        &b3,
        None,
        Some(Rect::new(
            block[2].x as i32 * 32,
            (block[2].y as i32 * 16) + offset,
            32,
            16,
        )),
    )
    .expect("draw rect");
}

fn main() {
    let width = game::WINDOW_WIDTH as u32;
    let height = game::WINDOW_HEIGHT as u32;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let mut timer_delay: u64;
    let mut cur_screen: i32 = 0;

    let window = video
        .window("Generic Puzzle Game", width, height)
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
    let game_over_surf = Surface::load_bmp("./img/gameover.bmp").unwrap();
    let game_over_texture = tc.create_texture_from_surface(game_over_surf).unwrap();
    let game_surf = Surface::load_bmp("./img/intro.bmp").unwrap();
    let game_texture = tc.create_texture_from_surface(game_surf).unwrap();
    let blocks = vec![
        "./img/block_black.bmp",
        "./img/block_clear.bmp",
        "./img/block_dblue.bmp",
        "./img/block_gray.bmp",
        "./img/block_green.bmp",
        "./img/block_ltblue.bmp",
        "./img/block_orange.bmp",
        "./img/block_pink.bmp",
        "./img/block_purple.bmp",
        "./img/block_red.bmp",
        "./img/block_yellow.bmp",
    ];

    let mut block_tex: Vec<sdl2::render::Texture> = Vec::new();
    for i in &blocks {
        let t_surf = Surface::load_bmp(i).unwrap();
        block_tex.push(tc.create_texture_from_surface(t_surf).unwrap());
    }

    let mut levels : Vec<sdl2::render::Texture> = Vec::new();
    for i in 1..=8 {
        let filename = format!("./img/level{}.bmp", i);
        let t_surf = Surface::load_bmp(filename).unwrap();
        levels.push(tc.create_texture_from_surface(t_surf).unwrap());
    }

    let logo = Surface::load_bmp("./img/lostlogo.bmp").unwrap();
    let lost_logo = tc.create_texture_from_surface(logo).unwrap();

    let mut e = sdl.event_pump().unwrap();
    can.set_draw_color(Color::RGB(0, 0, 0));
    can.clear();
    can.present();
    let mut grid: game::Grid = game::Grid::new(1280 / 32, 720 / 16);
    grid.new_piece();
    let mut font = ttf_context.load_font("./img/font.ttf", 18).expect("test");
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    let _text_surf = font
        .render("Score: ")
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    //let text_surf_tex = tc.create_texture_from_surface(&text_surf).unwrap();
    let mut prev_tick: u64 = 0;
    let mut tick_count: u64 = 0;
    let mut starting_image = false;
    'main: loop {
        if cur_screen == 0 {
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
                    } => cur_screen = 1,
                    _ => {}
                }
            }
            if starting_image == false {
                can.set_draw_color(Color::RGB(0, 0, 0));
                can.clear();
                can.copy(&lost_logo, None, None).expect("copy logo");

                let start = SystemTime::now();
                let se = start.duration_since(UNIX_EPOCH).expect("error on time");
                let tick = se.as_secs();
                if prev_tick == 0 {
                    prev_tick = tick;
                }
                if tick > prev_tick + 2 {
                    tick_count = 0;
                    prev_tick = 0;
                    starting_image = true;
                }
            } else {
                can.set_draw_color(Color::RGB(0, 0, 0));
                can.clear();
                can.copy(&game_texture, None, Some(Rect::new(0, 0, width, height)))
                    .expect("on copy");
            }
            can.present();
        } else if cur_screen == 1 {
            if grid.game_over == true {
                cur_screen = 2;
                grid.game_over = false;
            }
            // draw game screen
            let start = SystemTime::now();
            let se = start.duration_since(UNIX_EPOCH).expect("error on time");
            let tick = se.as_secs() * 1000 + se.subsec_nanos() as u64 / 1_000_000;
            let ptick = tick - prev_tick;
            prev_tick = tick;
            tick_count += ptick;
            let cur_level;
            timer_delay = match grid.score {
                0..=10 => {
                    cur_level = 1;
                    1500
                }
                11..=20 => {
                    cur_level = 2;
                    1200
                }
                21..=30 => {
                    cur_level = 3;
                    1000
                }
                31..=40 => {
                    cur_level = 4;
                    800
                }
                41..=50 => {
                    cur_level = 5;
                    700
                }
                51..=60 => {
                    cur_level = 6;
                    400
                }
                61..=65 => {
                    cur_level = 7;
                    300
                }
                66.. => {
                    cur_level = 8;
                    100
                }
            };

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
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } => grid.move_left(),
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => grid.move_right(),

                    Event::KeyDown {
                        keycode: Some(Keycode::Z),
                        ..
                    } => grid.shift_left(),
                    Event::KeyDown {
                        keycode: Some(Keycode::X),
                        ..
                    } => grid.shift_right(),
                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        ..
                    }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        ..
                    } => grid.swap_piece_colors(0),
                    Event::KeyDown {
                        keycode: Some(Keycode::S),
                        ..
                    } => grid.swap_piece_colors(1),
                    Event::KeyDown {
                        keycode: Some(Keycode::Down),
                        ..
                    } => grid.move_down(),
                    _ => {}
                }
            }
            can.set_draw_color(Color::RGB(0, 0, 0));
            can.clear();

            can.copy(&levels[cur_level-1], None, Some(Rect::new(0, 0, width, height)))
                .expect("on copy");
            draw_grid(&grid, &mut can, &block_tex);
            let score = format!("Score: {} Level: {}", grid.score, cur_level);
            let text_surf = font
                .render(&score)
                .blended(Color::RGB(255, 255, 255))
                .unwrap();
            let text_surf_tex = tc.create_texture_from_surface(&text_surf).unwrap();
            let TextureQuery {
                width: wi,
                height: hi,
                ..
            } = text_surf_tex.query();
            can.copy(
                &text_surf_tex,
                Some(Rect::new(0, 0, wi, hi)),
                Some(Rect::new(25, 25, wi, hi)),
            )
            .expect("on font copy");
            can.present();
        } else if cur_screen == 2 {
            // draw game over screen
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
                    } => cur_screen = 1,
                    _ => {}
                }
            }
            can.set_draw_color(Color::RGB(0, 0, 0));
            can.clear();
            can.copy(
                &game_over_texture,
                None,
                Some(Rect::new(0, 0, width, height)),
            )
            .expect("on copy");
            can.present();
        }
    }
}
