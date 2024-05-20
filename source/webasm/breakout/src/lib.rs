use wasm_bindgen::prelude::*;

// Import the JavaScript function
#[wasm_bindgen(module = "/index.js")]
extern "C" {
    pub fn random_range(min: i32, max: i32) -> i32;
}

// Example usage in Rust

pub const SCREEN_WIDTH: i32 = 1440;
pub const SCREEN_HEIGHT: i32 = 1080;
pub const TILE_W: usize = SCREEN_WIDTH as usize / 32;
pub const TILE_H: usize = SCREEN_HEIGHT as usize / 2 / 16;
pub const PADDLE_WIDTH: i32 = 200;
pub const PADDLE_HEIGHT: i32 = 20;
pub const BALL_SIZE: i32 = 16;
pub const BALL_SPEED: i32 = 5;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[wasm_bindgen]
impl Color {
    #[wasm_bindgen(constructor)]
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Paddle {
    x: i32,
    y: i32,
    color: Color,
}

#[wasm_bindgen]
impl Paddle {
    #[wasm_bindgen(constructor)]
    pub fn new(xpos: i32, ypos: i32, color: Color) -> Paddle {
        Paddle {
            x: xpos,
            y: ypos,
            color,
        }
    }

    #[wasm_bindgen]
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 10;
        }
    }

    #[wasm_bindgen]
    pub fn move_right(&mut self) {
        if self.x + PADDLE_WIDTH < SCREEN_WIDTH {
            self.x += 10;
        }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> i32 {
        self.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> i32 {
        self.y
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    #[wasm_bindgen(getter)]
    pub fn color(&self) -> Color {
        self.color.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Grid {
    blocks: Vec<u32>, // use u32 for color type
    colors: Vec<Color>,
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Grid {
        let blocks = vec![0; TILE_W* TILE_H]; // initialize all blocks with color_type 0
        let colors = Grid::rand_colors();
        Grid { blocks, colors }
    }

    fn rand_colors() -> Vec<Color> {
        let mut colors = vec![Color::new(0, 0, 0)];
        // Uncomment this if you can use the rand crate in wasm environment.
        // let mut rng = rand::thread_rng();
        for _ in 0..10 {
            colors.push(Color::new(
                random_range(0, 255) as u8,
                random_range(0, 255) as u8,
                random_range(0, 255) as u8,
            ));
        }
        colors
    }

    pub fn fill_rand(&mut self) {
        // Uncomment this if you can use the rand crate in wasm environment.
        // let mut rng = rand::thread_rng();
        for block in &mut self.blocks {
            *block = random_range(0, 8) as u32;
        }
        self.reset_colors();
    }

    fn reset_colors(&mut self) {
        self.colors = Grid::rand_colors();
    }

    #[wasm_bindgen]
    pub fn is_empty(&self) -> bool {
        for &block in &self.blocks {
            if block != 0 {
                return false;
            }
        }
        true
    }

    #[wasm_bindgen]
    pub fn get_block_color_type(&self, index: usize) -> u32 {
        self.blocks[index]
    }

    #[wasm_bindgen]
    pub fn set_block_color_type(&mut self, index: usize, color_type: u32) {
        self.blocks[index] = color_type;
    }

    #[wasm_bindgen(getter)]
    pub fn colors(&self) -> Vec<Color> {
        self.colors.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_colors(&mut self, colors: Vec<Color>) {
        self.colors = colors;
    }
    #[wasm_bindgen]
    pub fn index(&self, col: usize, row: usize) -> u32 {
        let idx = col * TILE_H + row;
        let block = self.blocks[idx];
        block
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Ball {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Default for Ball {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Ball {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Ball {
        Ball {
            x: SCREEN_WIDTH / 2,
            y: SCREEN_HEIGHT / 2 + 100,
            dx: BALL_SPEED,
            dy: -BALL_SPEED,
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.x = SCREEN_WIDTH / 2;
        self.y = SCREEN_HEIGHT / 2 + 100;
        // Uncomment this if you can use the rand crate in wasm environment.
        // let mut rng = rand::thread_rng();
        self.dx = if random_range(0, 10) >= 5 {
            BALL_SPEED
        } else {
            -BALL_SPEED
        };

        self.dy = -BALL_SPEED;
    }

    pub fn update(&mut self, paddle: &Paddle, grid: &mut Grid) -> UpdateResult {
        let mut score_delta = 0;
        let mut lives_delta = 0;
        self.x += self.dx;
        self.y += self.dy;

        if self.x <= 0 || self.x + BALL_SIZE >= SCREEN_WIDTH {
            self.dx = -self.dx;
        }

        if self.y <= 0 {
            self.dy = -self.dy;
        }

        if self.y + BALL_SIZE >= paddle.y()
            && self.x >= paddle.x()
            && self.x <= paddle.x() + PADDLE_WIDTH
        {
            self.dy = -self.dy;
        } else if self.y + BALL_SIZE > paddle.y() + BALL_SIZE {
            //std::thread::sleep(std::time::Duration::from_secs(1));
            lives_delta -= 1;
            self.reset();
        }

        let ball_left = self.x;
        let ball_right = self.x + BALL_SIZE;
        let ball_top = self.y;
        let ball_bottom = self.y + BALL_SIZE;

        for col in 0..TILE_W {
            for row in 0..TILE_H {
                let idx = col * TILE_H + row;
                if grid.get_block_color_type(idx) == 0 {
                    continue;
                }
                let block_x = col as i32 * 32;
                let block_y = row as i32 * 16;
                let block_right = block_x + 32;
                let block_bottom = block_y + 16;
                if ball_right > block_x
                    && ball_left < block_right
                    && ball_bottom > block_y
                    && ball_top < block_bottom
                {
                    self.dy = -self.dy;
                    grid.set_block_color_type(idx, 0);
                    score_delta += 10;
                    grid.reset_colors();
                }
            }
        }
        UpdateResult {
            score_delta,
            lives_delta,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> i32 {
        self.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> i32 {
        self.y
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    #[wasm_bindgen(getter)]
    pub fn dx(&self) -> i32 {
        self.dx
    }

    #[wasm_bindgen(setter)]
    pub fn set_dx(&mut self, dx: i32) {
        self.dx = dx;
    }

    #[wasm_bindgen(getter)]
    pub fn dy(&self) -> i32 {
        self.dy
    }

    #[wasm_bindgen(setter)]
    pub fn set_dy(&mut self, dy: i32) {
        self.dy = dy;
    }
}

#[wasm_bindgen]
pub struct UpdateResult {
    pub score_delta: i32,
    pub lives_delta: i32,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Breakout {
    paddle: Paddle,
    ball: Ball,
    score: u32,
    lives: u32,
    grid: Grid,
}

impl Default for Breakout {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Breakout {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Breakout {
            paddle: Paddle::new(
                SCREEN_WIDTH / 2 - PADDLE_WIDTH / 2,
                SCREEN_HEIGHT - 100,
                Color::new(255, 0, 0),
            ),
            ball: Ball::new(),
            score: 0,
            lives: 5,
            grid: Grid::new(),
        }
    }

    #[wasm_bindgen]
    pub fn new_game(&mut self) {
        self.grid.fill_rand();
        self.score = 0;
        self.lives = 5;
    }

    #[wasm_bindgen]
    pub fn update(&mut self) -> bool {
        let result = self.ball.update(&self.paddle, &mut self.grid);
        self.score = (self.score as i32 + result.score_delta) as u32;
        self.lives = (self.lives as i32 + result.lives_delta) as u32;

        if self.lives == 0 {
            self.new_game();
            return true;
        }
        if self.grid.is_empty() {
            self.new_game();
            return true;
        }
        false
    }

    #[wasm_bindgen(getter)]
    pub fn paddle(&self) -> Paddle {
        self.paddle.clone()
    }

    #[wasm_bindgen]
    pub fn set_paddle_xy(&mut self, x: i32, y: i32) {
        self.paddle.x = x;
        self.paddle.y = y;
    }

    #[wasm_bindgen(setter)]
    pub fn set_paddle(&mut self, paddle: &Paddle) {
        self.paddle = paddle.clone();
    }

    #[wasm_bindgen(getter)]
    pub fn ball(&self) -> Ball {
        self.ball.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_ball(&mut self, ball: Ball) {
        self.ball = ball;
    }

    #[wasm_bindgen(getter)]
    pub fn score(&self) -> u32 {
        self.score
    }

    #[wasm_bindgen(setter)]
    pub fn set_score(&mut self, score: u32) {
        self.score = score;
    }

    #[wasm_bindgen(getter)]
    pub fn lives(&self) -> u32 {
        self.lives
    }

    #[wasm_bindgen(setter)]
    pub fn set_lives(&mut self, lives: u32) {
        self.lives = lives;
    }

    #[wasm_bindgen(getter)]
    pub fn grid(&self) -> Grid {
        self.grid.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_grid(&mut self, grid: Grid) {
        self.grid = grid;
    }
}
// Example usage function in Rust
#[wasm_bindgen]
pub fn example_random_range_usage() -> i32 {
    random_range(1, 100)
}