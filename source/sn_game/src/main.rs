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
pub const TILE_SIZE: usize = 16;
pub const SIZE: i32 = 16;
pub const TILE_W: usize = 1280 / TILE_SIZE;
pub const TILE_H: usize = 720 / TILE_SIZE;
pub const WIDTH: i32 = 1280 / SIZE;
pub const HEIGHT: i32 = 720 / SIZE;

/// game Grid
struct Grid {
    pub blocks: Box<[[u8; TILE_H]; TILE_W]>,
    pub score: i32,
    pub lives: i32,
    apple_num: i32,
    apple_count: i32,
}

impl Grid {
    /// create a new grid
    pub fn new() -> Grid {
        let g = Box::new([[0; TILE_H]; TILE_W]);
        Grid {
            blocks: g,
            score: 0,
            lives: 4,
            apple_count: 1,
            apple_num: 1,
        }
    }
    /// reset game lives and apple count
    pub fn reset_lives(&mut self) {
        self.score = 0;
        self.lives = 4;
        self.apple_count = 1;
        self.apple_num = 1;
    }
    /// clear the grid and reset lives
    pub fn clear(&mut self) {
        for i in 0..TILE_W {
            for z in 0..TILE_H {
                self.blocks[i][z] = 0;
            }
        }
        self.reset_lives();
        let apple = self.rand_apple();
        self.set_apple(apple);
    }

    /// generate a random apple
    pub fn rand_apple(&mut self) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        let ix = rng.gen_range(2..WIDTH - 2);
        let iy = rng.gen_range(2..HEIGHT - 2);
        if self.blocks[ix as usize][iy as usize] == 2 {
            return self.rand_apple();
        }
        return (ix as usize, iy as usize);
    }

    /// set apple in grid
    pub fn set_apple(&mut self, apple: (usize, usize)) {
        self.blocks[apple.0][apple.1] = 2;
    }

    /// check if the snake colided with any apples
    pub fn check_apples(&mut self, i: usize, z: usize, snake: &mut Snake) {
        for a in &snake.sn {
            if a.x as usize == i && a.y as usize == z {
                snake.grow();
                self.blocks[i][z] = 0;
                self.apple_num -= 1;
                if self.apple_num == 0 {
                    self.apple_count += 1;
                    self.apple_num = self.apple_count;
                    for _ in 0..self.apple_num {
                        let apple = self.rand_apple();
                        self.set_apple(apple);
                    }
                }
                self.score += 1;
                break;
            }
        }
    }

    /// draw the apples on the grid to the screen and check if the snake colides with them
    pub fn draw_grid(
        &mut self,
        snake: &mut Snake,
        can: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) {
        for i in 0..TILE_W {
            for z in 0..TILE_H {
                let color = match self.blocks[i][z] {
                    0 => Color::RGB(0, 0, 0),
                    1 => Color::RGB(255, 255, 255),
                    2 => {
                        self.check_apples(i, z, snake);
                        Color::RGB(255, 0, 0)
                    }
                    _ => Color::RGB(0, 0, 0),
                };
                can.set_draw_color(color);
                can.fill_rect(Some(Rect::new(
                    i as i32 * SIZE,
                    z as i32 * SIZE,
                    SIZE as u32,
                    SIZE as u32,
                )))
                .expect("on fill");
            }
        }

        snake.draw_snake(can);
    }
    /// the snake died (lost a life)
    pub fn minus_life(&mut self) {
        self.lives -= 1;
        if self.lives <= 0 {
            self.clear();
        }
    }

    /// update grid snake logic
    pub fn update(&mut self, snake: &mut Snake) {
        let tail = snake.sn.get(snake.sn.len() - 1).cloned().unwrap();
        if snake.check_out() {
            snake.reset_snake();
            self.minus_life();
            return;
        }
        snake.move_snake(&tail);
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
#[derive(PartialEq)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

/// game's snake structure
struct Snake {
    pub direction: Dir,
    pub sn: VecDeque<Point>,
}

impl Snake {
    /// create a new snake
    fn new() -> Self {
        Snake {
            direction: Dir::Right,
            sn: VecDeque::from(vec![Point { x: 10, y: 10 }]),
        }
    }
    /// check for duplicate parts of the snake
    fn duplicates(&mut self) -> bool {
        let top = self.sn.get(self.sn.len() - 1).cloned().unwrap();
        for ix in 0..self.sn.len() - 1 {
            let i = self.sn.get(ix).cloned().unwrap();
            if top.x == i.x && top.y == i.y {
                return true;
            }
        }
        false
    }
    /// check if the snake is out of bounds
    fn check_out(&mut self) -> bool {
        for i in self.sn.iter() {
            if i.x <= 0 || i.x > WIDTH - 1 {
                return true;
            }
            if i.y <= 0 || i.y > HEIGHT - 1 {
                return true;
            }
        }
        if self.duplicates() {
            return true;
        }
        false
    }
    /// grow snake
    pub fn grow(&mut self) {
        let tail = self.sn.get(self.sn.len() - 1).cloned().unwrap();
        self.grow_tail(&tail);
    }
    /// grow snake by tail
    pub fn grow_tail(&mut self, tail: &Point) {
        match self.direction {
            Dir::Left => {
                self.sn.push_back(Point::new(tail.x - 1, tail.y));
            }
            Dir::Right => {
                self.sn.push_back(Point::new(tail.x + 1, tail.y));
            }
            Dir::Up => {
                self.sn.push_back(Point::new(tail.x, tail.y - 1));
            }
            Dir::Down => {
                self.sn.push_back(Point::new(tail.x, tail.y + 1));
            }
        }
    }
    /// move snake in direction
    pub fn move_snake(&mut self, tail: &Point) {
        self.sn.pop_front();
        self.grow_tail(tail);
    }
    /// reset snake back to 1 block
    pub fn reset_snake(&mut self) {
        self.sn.clear();
        self.sn.push_back(Point::new(10, 10));
        self.direction = Dir::Right;
    }

    /// draw the snake to the screen
    pub fn draw_snake(&mut self, can: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        for i in &self.sn {
            can.set_draw_color(Color::RGB(0, 255, 0));
            can.fill_rect(Some(Rect::new(
                i.x * SIZE,
                i.y * SIZE,
                SIZE as u32,
                SIZE as u32,
            )))
            .expect("on fill");
        }
    }

    /// move snake left
    pub fn move_left(&mut self) {
        if self.direction != Dir::Right {
            self.direction = Dir::Left;
        }
    }

    /// move snake right
    pub fn move_right(&mut self) {
        if self.direction != Dir::Left {
            self.direction = Dir::Right;
        }
    }

    /// move snake down
    pub fn move_down(&mut self) {
        if self.direction != Dir::Up {
            self.direction = Dir::Down;
        }
    }

    /// move snake up
    pub fn move_up(&mut self) {
        if self.direction != Dir::Down {
            self.direction = Dir::Up;
        }
    }
}

/// main function
fn main() {
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
    let mut snake: Snake = Snake::new();
    let mut prev_tick: u64 = 0;
    let mut tick_count = 0;
    let apple = grid.rand_apple();
    grid.set_apple(apple);
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
                    snake.move_left();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    snake.move_right();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    snake.move_down();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    snake.move_up();
                }
                _ => {}
            }
        }
        can.clear();
        grid.draw_grid(&mut snake, &mut can);
        let turn_surf = font
            .render(&format!("Score: {} Lives: {}", grid.score, grid.lives))
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
            grid.update(&mut snake);
        }
    }
}
