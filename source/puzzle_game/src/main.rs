// cargo run --release

mod puzzle;
mod scores;

use puzzle::game;
use rand::Rng;
use scores::high_scores;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::surface::Surface;
use std::time::{SystemTime, UNIX_EPOCH};

/// draw the games grid on screen
fn draw_grid(
    grid: &game::Grid,
    can: &mut sdl2::render::Canvas<sdl2::video::Window>,
    blocks:&[sdl2::render::Texture],
) {
    let offset = 0;
    for x in 0..grid.get_width() as usize {
        for y in 0..grid.get_height() as usize {
            let color = grid.get_grid_point(x, y);
            if color >= 1 {
                let b = blocks.get(color as usize).unwrap();
                can.copy(
                    b,
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
        b,
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
        b2,
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
        b3,
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

/// main function contains game code 
fn main() {
    let width = game::WINDOW_WIDTH as u32;
    let height = game::WINDOW_HEIGHT as u32;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let mut timer_delay: u64;
    let mut cur_screen: i32 = 0;

    let icon = Surface::load_bmp("./img/icon.bmp").unwrap();

    let mut window = video
        .window("Generic Puzzle Game", width, height)
        .opengl()
        .build()
        .unwrap();

    window.set_icon(icon);

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

    let mut levels: Vec<sdl2::render::Texture> = Vec::new();
    for i in 1..=8 {
        let filename = format!("./img/level{}.bmp", i);
        let t_surf = Surface::load_bmp(filename).unwrap();
        levels.push(tc.create_texture_from_surface(t_surf).unwrap());
    }

    let logo = Surface::load_bmp("./img/lostlogo.bmp").unwrap();
    let lost_logo = tc.create_texture_from_surface(logo).unwrap();

    let game_over_logo = Surface::load_bmp("./img/gameover_logo.bmp").unwrap();
    let game_over_logo_ex = tc.create_texture_from_surface(game_over_logo).unwrap();

    let lost_game_over_logo = Surface::load_bmp("./img/lostsidedead_logo.bmp").unwrap();
    let lost_game_over_logo_ex = tc.create_texture_from_surface(lost_game_over_logo).unwrap();

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
    let mut game_over_score = 0;
    let mut score_shown = true;
    let mut score_menu = high_scores::ScoreMenu::new();
    score_menu.load();

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

            can.set_draw_color(Color::RGB(0, 0, 0));
            can.clear();
          
            if !starting_image {
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
                can.copy(&game_texture, None, Some(Rect::new(0, 0, width, height)))
                    .expect("on copy");
            }
            can.present();
        } else if cur_screen == 1 {
            if grid.game_over  {
                cur_screen = 2;
                grid.game_over = false;
            } else {
                game_over_score = grid.score;
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

            can.copy(
                &levels[cur_level - 1],
                None,
                Some(Rect::new(0, 0, width, height)),
            )
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
                    Event::KeyDown { keycode: key, .. } => {
                        if key == Some(Keycode::Backspace) {
                            score_menu.input.pop();
                        }
                        if key == Some(Keycode::Return) {
                            //enter
                            if !score_menu.input.is_empty() {
                                let s = (String::from(&score_menu.input), game_over_score);
                                score_menu.scores.push(s);
                                score_menu.sort_scores();
                                score_menu.input = String::new();
                                game_over_score = 0;
                            }
                        }
                        if key == Some(Keycode::Space) {
                            if score_shown  && game_over_score == 0 {
                                score_shown = false;
                            } else {
                                cur_screen = 1;
                                score_shown = true;
                                println!("here\n");
                            }
                        }
                    }
                    Event::TextInput {
                        timestamp: _,
                        window_id: _,
                        text: s,
                    } => {
                        if (game_over_score > 0 && score_menu.input.len() < 10)
                            && (score_menu.scores.len() < 10
                                || score_menu.scores[9].1 < game_over_score)
                        {
                            score_menu.type_key(&s);
                        }
                    }
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

            if score_shown  {
                can.fill_rect(Some(Rect::new(25, 25, 1280 - 50, 720 - 50)))
                    .expect("on fill");

                let TextureQuery {
                    width: wix,
                    height: hix,
                    ..
                } = game_over_logo_ex.query();
                can.copy(
                    &game_over_logo_ex,
                    Some(Rect::new(0, 0, wix, hix)),
                    Some(Rect::new(640, 50, 500, 250)),
                )
                .expect("on logo copy");

                let TextureQuery {
                    width: wix1,
                    height: hix1,
                    ..
                } = lost_game_over_logo_ex.query();
                can.copy(
                    &lost_game_over_logo_ex,
                    Some(Rect::new(0, 0, wix1, hix1)),
                    Some(Rect::new(1280 / 2 - 295, 580, wix1, hix1)),
                )
                .expect("on logo copy");

                let mut pos_y = 75;
                let mut index = 0;

                let text_surf1 = font
                    .render("High Scores")
                    .blended(Color::RGB(255, 0, 0))
                    .unwrap();
                let text_surf_tex1 = tc.create_texture_from_surface(&text_surf1).unwrap();
                let TextureQuery {
                    width: wi,
                    height: hi,
                    ..
                } = text_surf_tex1.query();

                can.copy(
                    &text_surf_tex1,
                    Some(Rect::new(0, 0, wi, hi)),
                    Some(Rect::new(55, 40, wi, hi)),
                )
                .expect("on font copy");

                for i in &score_menu.scores {
                    if index >= 9 {
                        break;
                    }
                    index += 1;

                    let score = format!("Score: {} : {}", i.1, i.0);
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
                        Some(Rect::new(100, pos_y, wi, hi)),
                    )
                    .expect("on font copy");
                    pos_y += 25;
                }

                let score;

                if game_over_score > 0
                    && (score_menu.scores.len() < 10 || score_menu.scores[9].1 < game_over_score)
                {
                    score = format!(
                        "Your Score: {} Enter your name and press Enter: - {}",
                        game_over_score, score_menu.input
                    );
                } else {
                    score = "Press Space to Exit High Scores".to_string();
                }
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
                    Some(Rect::new(1280 / 2 - 100, 720 / 2 - 25, wi, hi)),
                )
                .expect("on font copy");
            }

            can.present();
        }
    }
    score_menu.save();
}
