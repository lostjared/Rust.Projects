//! Basic Snake Game
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};
/// public constants
pub const TILE_W: usize = 1280 / 8;
pub const TILE_H: usize = 720 / 8;
pub const WIDTH: i32 = 1280 / 8;
pub const HEIGHT: i32 = 720 / 8;

/// game Grid
struct Grid {
    pub blocks: Box<[[u8; TILE_H]; TILE_W]>,
}

impl Grid {
    fn new() -> Grid {
        let g = Box::new([[0; TILE_H]; TILE_W]);
        Grid { blocks: g }
    }

    fn clear(&mut self) {
        for i in 0..TILE_W {
            for z in 0..TILE_H {
                self.blocks[i][z] = 0;
            }
        }
    }
}
/// Point on the Screen
#[derive(Clone, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(xi: i32, yi: i32) -> Point {
        Point { x: xi, y: yi }
    }
}
/// Current Snake Direction
enum Dir {
    Left,
    Right,
    Down,
    Up,
}
/// main function
fn main() {
    let mut score = 0;
    let mut lives = 4;
    let width = 1280;
    let height = 720;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("Snake", width, height)
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
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let tc = can.texture_creator();
    let font = ttf_context.load_font("./font.ttf", 18).expect("test");
    let _text_surf = font
        .render("Hello, World!")
        .blended(Color::RGB(255, 255, 255))
        .unwrap();

    let mut grid: Grid = Grid::new();
    let mut sn: VecDeque<Point> = VecDeque::new();
    let mut direction: Dir = Dir::Down;
    sn.push_back(Point::new(10, 10));

    let mut prev_tick: u64 = 0;
    let mut tick_count = 0;
    let mut pos: Point = Point::new(10, 10);

    let mut rng = rand::thread_rng();
    let ix = rng.gen_range(2..WIDTH - 2);
    let iy = rng.gen_range(2..HEIGHT - 2);
    let mut apple_num = 1;
    let mut apple_count = 1;

    grid.blocks[ix as usize][iy as usize] = 2;

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
                    direction = Dir::Left;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    direction = Dir::Right;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    direction = Dir::Down;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    direction = Dir::Up;
                }
                _ => {}
            }
        }
        can.clear();

        for i in 0..TILE_W {
            for z in 0..TILE_H {
                let color;
                match grid.blocks[i][z] {
                    0 => {
                        color = Color::RGB(0, 0, 0);
                    }
                    1 => {
                        color = Color::RGB(255, 255, 255);
                    }
                    2 => {
                        color = Color::RGB(255, 0, 0);

                        for a in &sn {
                            if a.x as usize == i && a.y as usize == z {
                                let tail = sn.get(sn.len() - 1).cloned().unwrap();
                                match direction {
                                    Dir::Left => {
                                        sn.push_back(Point::new(tail.x - 1, tail.y));
                                    }
                                    Dir::Right => {
                                        sn.push_back(Point::new(tail.x + 1, tail.y));
                                    }
                                    Dir::Up => {
                                        sn.push_back(Point::new(tail.x, tail.y - 1));
                                    }
                                    Dir::Down => {
                                        sn.push_back(Point::new(tail.x, tail.y + 1));
                                    }
                                }
                                grid.blocks[i][z] = 0;
                                apple_num -= 1;
                                if apple_num == 0 {
                                    apple_count += 1;
                                    apple_num = apple_count;
                                    for _ in 0..apple_num {
                                        let apple = rand_apple(&grid);
                                        grid.blocks[apple.0][apple.1] = 2;
                                    }
                                }
                                 score += 1;
                                break;
                            }
                        }
                    }
                    _ => {
                        color = Color::RGB(0, 0, 0);
                    }
                }
                can.set_draw_color(color);
                can.fill_rect(Some(Rect::new(i as i32 * 8, z as i32 * 8, 8, 8)))
                    .expect("on fill");
            }
        }

        for i in &sn {
            can.set_draw_color(Color::RGB(0, 255, 0));
            can.fill_rect(Some(Rect::new(i.x * 8, i.y * 8, 8, 8)))
                .expect("on fill");
        }

        let turn_surf = font
            .render(&format!("Score: {} Lives: {}", score, lives))
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
        let tail = sn.get(sn.len() - 1).cloned().unwrap();
        if tick_count > 50 {
            tick_count = 0;

            match direction {
                Dir::Left => {
                    if check_out(&pos, &sn) {
                        sn.clear();
                        sn.push_back(Point::new(10, 10));
                        pos.x = 10;
                        pos.y = 10;
                        direction = Dir::Right;
                        lives -= 1;
                        if lives <= 0 {
                            score = 0;
                            lives = 4;
                            grid.clear();
                            let apple = rand_apple(&grid);
                            grid.blocks[apple.0][apple.1] = 2;
                        }
                        continue;
                    }
                    sn.pop_front();
                    sn.push_back(Point::new(tail.x - 1, tail.y));
                    pos.x -= 1;
                }
                Dir::Right => {
                    if check_out(&pos, &sn) {
                        sn.clear();
                        sn.push_back(Point::new(10, 10));
                        pos.x = 10;
                        pos.y = 10;
                        direction = Dir::Right;
                        lives -= 1;
                        if lives <= 0 {
                            score = 0;
                            lives = 4;
                            grid.clear();
                            let apple = rand_apple(&grid);
                            grid.blocks[apple.0][apple.1] = 2;
                        }
                        continue;
                    }
                    sn.pop_front();
                    sn.push_back(Point::new(tail.x + 1, tail.y));
                    pos.x += 1;
                }
                Dir::Down => {
                    if check_out(&pos, &sn) {
                        sn.clear();
                        sn.push_back(Point::new(10, 10));
                        pos.x = 10;
                        pos.y = 10;
                        direction = Dir::Right;
                        lives -= 1;
                        if lives <= 0 {
                            score = 0;
                            lives = 4;
                            grid.clear();
                            let apple = rand_apple(&grid);
                            grid.blocks[apple.0][apple.1] = 2;
                        }
                        continue;
                    }
                    sn.pop_front();
                    sn.push_back(Point::new(tail.x, tail.y + 1));
                    pos.y += 1;
                }
                Dir::Up => {
                    if check_out(&pos, &sn) {
                        sn.clear();
                        sn.push_back(Point::new(10, 10));
                        pos.x = 10;
                        pos.y = 10;
                        direction = Dir::Right;
                        lives -= 1;
                        if lives <= 0 {
                            score = 0;
                            lives = 4;
                            grid.clear();
                            let apple = rand_apple(&grid);
                            grid.blocks[apple.0][apple.1] = 2;
                        }
                        continue;
                    }
                    sn.pop_front();
                    sn.push_back(Point::new(tail.x, tail.y - 1));
                    pos.y -= 1;
                }
            }
        }
    }
}
/// check for duplicate parts of the snake
fn duplicates(pos: &VecDeque<Point>) -> bool {
    let top = pos.get(0).cloned().unwrap();
    for i in pos.iter().skip(1) {
        if top.x == i.x && top.y == i.y {
            return true;
        }
    }
    false
}
/// check if the snake is out of bounds
fn check_out(_cur_point: &Point, pos: &VecDeque<Point>) -> bool {
    for i in pos.iter() {
        if i.x <= 0 || i.x > WIDTH - 1 {
            return true;
        }
        if i.y <= 0 || i.y > HEIGHT - 1 {
            return true;
        }
    }
    if duplicates(pos) {
        return true;
    }
    false
}

fn rand_apple(g: &Grid) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let ix = rng.gen_range(2..WIDTH - 2);
    let iy = rng.gen_range(2..HEIGHT - 2);
    if g.blocks[ix as usize][iy as usize] == 2 {
        return rand_apple(g);
    }
    return (ix as usize, iy as usize);
}