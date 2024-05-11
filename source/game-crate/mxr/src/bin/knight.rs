// draw to offscreen 640x480 surface
// copy surface to screen as screen as 1440x1080 or width height as arguments

use mxr::mxr::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const BOARD_SIZE: usize = 8;

#[derive(Clone, Copy)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn new(row: i32, col: i32) -> Self {
        Position { row, col }
    }
}

fn main() -> Result<(), String> {
    let mut width = 1440;
    let mut height = 1080;
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 {
        width = args[1].parse::<u32>().unwrap();
        height = args[2].parse::<u32>().unwrap();
    }
    let mut mx = MXWindowBuilder::new()
        .create("Knights Tour", width, height)
        .build()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("./data/font.ttf", 18)?;
    let tc = mx.can.texture_creator();
    let files = vec!["./data/logo.bmp", "./data/knight.bmp"];
    let textures = mx.load_gfx(files, &tc, Some(sdl2::pixels::Color::RGB(255, 255, 255)))?;
    let tex = mx
        .printtext_texture(
            &font,
            &tc,
            sdl2::pixels::Color::RGB(255, 255, 255),
            &format!("Press Space to Move Knight"),
        )
        .unwrap();
    let tex_over = mx
        .printtext_texture(
            &font,
            &tc,
            sdl2::pixels::Color::RGB(255, 255, 255),
            "Tour Complete",
        )
        .unwrap();
    let tex_s = tex_get_size(&tex);
    let tex_over_s = tex_get_size(&tex_over);
    let mut texture = tc
        .create_texture_target(tc.default_pixel_format(), 640, 480)
        .unwrap();

    let mut tour_over = false;
    let mut moves = 0;
    let startx = 100;
    let starty = 30;
    let mut board: [[i32; BOARD_SIZE]; BOARD_SIZE] = [[0; BOARD_SIZE]; BOARD_SIZE];
    let htable: [[i32; BOARD_SIZE]; BOARD_SIZE] = [
        [2, 3, 4, 4, 4, 4, 3, 2],
        [3, 4, 6, 6, 6, 6, 4, 3],
        [4, 6, 8, 8, 8, 8, 6, 4],
        [4, 6, 8, 8, 8, 8, 6, 4],
        [4, 6, 8, 8, 8, 8, 6, 4],
        [4, 6, 8, 8, 8, 8, 6, 4],
        [3, 4, 6, 6, 6, 6, 4, 3],
        [2, 3, 4, 4, 4, 4, 3, 2],
    ];
    let horizontal: [i32; 8] = [2, 1, -1, -2, -2, -1, 1, 2];
    let vertical: [i32; 8] = [-1, -2, -2, -1, 1, 2, 2, 1];
    let mut knight_pos = Position::new(1, 6);

    fn drawboard(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, startx: i32, starty: i32, board: &[[i32; BOARD_SIZE]; BOARD_SIZE]) {
        let mut dx = startx;
        let mut dy = starty;
        let mut ion = true;
        for i in 0..BOARD_SIZE {
            for z in 0..BOARD_SIZE {
                let color = if ion {
                    sdl2::pixels::Color::RGB(255, 255, 255)
                } else {
                    sdl2::pixels::Color::RGB(255, 0, 0)
                };
                ion = !ion;

                if board[i][z] == 0 {
                    canvas.set_draw_color(color);
                    canvas
                        .fill_rect(sdl2::rect::Rect::new(dx, dy, 50, 50))
                        .expect("on drawing rectangle for grid");
                }

                dx += 55;
                if dx >= startx + 8 * 55 {
                    dx = startx;
                    dy += 55;
                    ion = !ion;
                }
            }
        }
    }

    fn drawknight(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, startx: i32, starty: i32, knight_pos: Position, texture: &sdl2::render::Texture) {
        let dx = startx + knight_pos.col * 55;
        let dy = starty + knight_pos.row * 55;
        canvas
            .copy(
                texture,
                None,
                sdl2::rect::Rect::new(dx + 5, dy + 5, 35, 35),
            )
            .expect("on draw knight");
    }

    fn clearboard(board: &mut [[i32; BOARD_SIZE]; BOARD_SIZE]) {
        for i in 0..BOARD_SIZE {
            for z in 0..BOARD_SIZE {
                board[i][z] = 0;
            }
        }
    }

    fn nextmove(
        board: &mut [[i32; BOARD_SIZE]; BOARD_SIZE],
        htable: &[[i32; BOARD_SIZE]; BOARD_SIZE],
        horizontal: &[i32; 8],
        vertical: &[i32; 8],
        knight_pos: &mut Position,
        moves: &mut i32,
        tour_over: &mut bool,
    ) {
        let mut smallest = 100;
        let mut choice = -1;

        for i in 0..8 {
            let mut row = knight_pos.row;
            let mut col = knight_pos.col;
            row += horizontal[i];
            col += vertical[i];
            if row >= 0 && row < 8 && col >= 0 && col < 8 && board[row as usize][col as usize] == 0 {
                if htable[row as usize][col as usize] < smallest && htable[row as usize][col as usize] != 0 {
                    smallest = htable[row as usize][col as usize];
                    choice = i as i32;
                }
            }
        }

        if choice != -1 {
            board[knight_pos.row as usize][knight_pos.col as usize] = 1;
            knight_pos.row += horizontal[choice as usize];
            knight_pos.col += vertical[choice as usize];
            *moves += 1;
            if *moves == 63 {
                *moves += 1;
                board[knight_pos.row as usize][knight_pos.col as usize] = 1;
                *tour_over = true;
            }
        }
    }

    clearboard(&mut board);

    'main: loop {
        for event in mx.event.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    nextmove(&mut board, &htable, &horizontal, &vertical, &mut knight_pos, &mut moves, &mut tour_over);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    clearboard(&mut board);
                    knight_pos = Position::new(1, 6);
                    tour_over = false;
                    moves = 0;
                }
                _ => {}
            }
        }
        mx.can
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                texture_canvas.clear();
                drawboard(texture_canvas, startx, starty, &board);
                drawknight(texture_canvas, startx, starty, knight_pos, &textures[1]);
                if !tour_over {
                    texture_canvas
                        .copy(&tex, None, sdl2::rect::Rect::new(5, 5, tex_s.0, tex_s.1))
                        .expect("on copy");
                } else {
                    texture_canvas
                        .copy(
                            &tex_over,
                            None,
                            sdl2::rect::Rect::new(5, 5, tex_over_s.0, tex_over_s.1),
                        )
                        .expect("on copy");
                }
            })
            .map_err(|x| x.to_string())?;
        mx.can.clear();
        mx.can.copy(&texture, None, None)?;
        mx.can.present();
    }
    Ok(())
}