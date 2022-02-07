use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{SystemTime, UNIX_EPOCH};


static WIDTH: u32 = 1280;
static HEIGHT: u32 = 720;

struct Paddle {
    pub pos_x: i32,
    pub pos_y: i32,
    pub pos_w: u32,
    pub pos_h: u32,
}

impl Paddle {
    fn new() -> Paddle {
        Paddle {
            pos_x: 10,
            pos_y: 0,
            pos_w: 20,
            pos_h: 100,
        }
    }
}

struct Ball {
    pub pos_x: i32,
    pub pos_y: i32,
    pub dir: u32,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            pos_x: 640 / 2,
            pos_y: 480 / 2,
            dir: 1,
        }
    }
}

fn proc_game(one: &mut Paddle, ball: &mut Ball) {
    let speed = 5;
    let mut rng = rand::thread_rng();
    if ball.dir == 1 && ball.pos_x > 5 && ball.pos_y > 17 {
        if ball.pos_x == one.pos_x + 10 && ball.pos_y >= one.pos_y && ball.pos_y <= one.pos_y + 100
        {
            ball.dir = rng.gen_range(0..2) + 3;
        } else {
            ball.pos_x -= speed;
            ball.pos_y -= speed;
        }
    } else if ball.dir == 2 && ball.pos_x > 5 && ball.pos_y < HEIGHT as i32 {
        if ball.pos_x == one.pos_x + 10 && ball.pos_y >= one.pos_y && ball.pos_y <= one.pos_y + 100
        {
            ball.dir = rng.gen_range(0..2) + 3;
        } else {
            ball.pos_x -= speed;
            ball.pos_y += speed;
        }
    } else if ball.dir == 3 && ball.pos_x < WIDTH as i32 && ball.pos_y > 17 {
        if ball.pos_x > WIDTH as i32-20 {
            ball.dir = rng.gen_range(0..2) + 1;
        } else {
            ball.pos_x += speed;
            ball.pos_y -= speed;
        }
    } else if ball.dir == 4 && ball.pos_x < WIDTH as i32 && ball.pos_y < HEIGHT as i32 {
        if ball.pos_x > WIDTH as i32-20 {
            ball.dir = rng.gen_range(0..2) + 1;
        } else {
            ball.pos_x += speed;
            ball.pos_y += speed;
        }
    } else if ball.dir == 1 || ball.dir == 3 {
        ball.dir += 1;
    } else if ball.dir == 2 || ball.dir == 4 {
        ball.dir -= 1;
    }

    if ball.pos_x < 6 {
        ball.pos_x = WIDTH as i32/2;
        ball.pos_y = HEIGHT as i32/2;
        ball.dir = rng.gen_range(0..4) + 1;
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}

fn main() {
    let width = WIDTH;
    let height = HEIGHT;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("WallPong", width, height)
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
    let mut paddle: Paddle = Paddle::new();
    let mut ball: Ball = Ball::new();
    let mut prev_tick = 0;
    let mut tick_count = 0;
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
        can.set_draw_color(Color::RGB(255, 255, 255));
        can.fill_rect(Some(Rect::new(
            paddle.pos_x,
            paddle.pos_y,
            paddle.pos_w,
            paddle.pos_h,
        )))
        .expect("on fill");
        can.fill_rect(Some(Rect::new(width as i32 - 10, 0, 10, height)))
            .expect("on fill");
        can.fill_rect(Some(Rect::new(ball.pos_x, ball.pos_y, 10, 10)))
            .expect("on fill");
        can.present();
        let start = SystemTime::now();
        let se = start.duration_since(UNIX_EPOCH).expect("error on time");
        let tick = se.as_secs() * 1000 + se.subsec_nanos() as u64 / 1_000_000;
        let ptick = tick - prev_tick;
        prev_tick = tick;
        tick_count += ptick;
        if tick_count > 15 {
            if e.keyboard_state().is_scancode_pressed(Scancode::Up) && paddle.pos_y > 0 {
                paddle.pos_y -= 10;
            }

            if e.keyboard_state().is_scancode_pressed(Scancode::Down)
                && paddle.pos_y + (paddle.pos_h as i32) < height as i32
            {
                paddle.pos_y += 10;
            }
            tick_count = 0;
            proc_game(&mut paddle, &mut ball);
        }
    }
}
