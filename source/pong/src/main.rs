use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{SystemTime, UNIX_EPOCH};

static WIDTH: u32 = 1280;
static HEIGHT: u32 = 720;

#[derive(Debug)]
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

    fn new2() -> Paddle {
        Paddle {
            pos_x: 1260,
            pos_y: 0,
            pos_w: 20,
            pos_h: 100,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    UpLeft,    
    DownLeft,  
    UpRight,   
    DownRight  
}

#[derive(Debug)]
struct Ball {
    pub pos_x: i32,
    pub pos_y: i32,
    pub dir: Direction,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            pos_x: WIDTH as i32 / 2,
            pos_y: HEIGHT as i32 / 2,
            dir: Direction::DownRight, 
        }
    }
}

fn proc_game(one: &mut Paddle, two: &mut Paddle, ball: &mut Ball) {
    let speed = 5;
    let mut rng = rand::thread_rng();

    match ball.dir {
        Direction::UpLeft => {
            ball.pos_x -= speed;
            ball.pos_y -= speed;
        },
        Direction::DownLeft => {
            ball.pos_x -= speed;
            ball.pos_y += speed;
        },
        Direction::UpRight => {
            ball.pos_x += speed;
            ball.pos_y -= speed;
        },
        Direction::DownRight => {
            ball.pos_x += speed;
            ball.pos_y += speed;
        }
    }

    if ball.pos_x <= one.pos_x + one.pos_w as i32 && 
       ball.pos_y >= one.pos_y && ball.pos_y <= one.pos_y + one.pos_h as i32 {
        ball.dir = match ball.dir {
            Direction::UpLeft => if rng.gen_bool(0.5) { Direction::UpRight } else { Direction::DownRight },
            Direction::DownLeft => if rng.gen_bool(0.5) { Direction::UpRight } else { Direction::DownRight },
            _ => ball.dir,
        };
    } else if ball.pos_x >= two.pos_x && 
              ball.pos_y >= two.pos_y && ball.pos_y <= two.pos_y + two.pos_h as i32 {
        ball.dir = match ball.dir {
            Direction::UpRight => if rng.gen_bool(0.5) { Direction::UpLeft } else { Direction::DownLeft },
            Direction::DownRight => if rng.gen_bool(0.5) { Direction::UpLeft } else { Direction::DownLeft },
            _ => ball.dir,
        };
    }

    if ball.pos_y <= 0 {  
        ball.dir = match ball.dir {
            Direction::UpLeft => Direction::DownLeft,
            Direction::UpRight => Direction::DownRight,
            _ => ball.dir,
        };
    } else if ball.pos_y >= HEIGHT as i32 - 10 {  
        ball.dir = match ball.dir {
            Direction::DownLeft => Direction::UpLeft,
            Direction::DownRight => Direction::UpRight,
            _ => ball.dir,
        };
    }

    if ball.pos_x < 0 || ball.pos_x > WIDTH as i32 {
        ball.pos_x = WIDTH as i32 / 2;
        ball.pos_y = HEIGHT as i32 / 2;
        ball.dir = match rng.gen_range(0..4) {
            0 => Direction::UpLeft,
            1 => Direction::DownLeft,
            2 => Direction::UpRight,
            3 => Direction::DownRight,
            _ => unreachable!(),
        };
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
    let mut paddle_two: Paddle = Paddle::new2();
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

        can.set_draw_color(Color::RGB(255, 255, 255));
        can.fill_rect(Some(Rect::new(
            paddle_two.pos_x,
            paddle_two.pos_y,
            paddle_two.pos_w,
            paddle_two.pos_h,
        )))
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

            if e.keyboard_state().is_scancode_pressed(Scancode::A) && paddle_two.pos_y > 0 {
                paddle_two.pos_y -= 10;
            }

            if e.keyboard_state().is_scancode_pressed(Scancode::S)
                && paddle_two.pos_y + (paddle_two.pos_h as i32) < height as i32
            {
                paddle_two.pos_y += 10;
            }

            tick_count = 0;
            proc_game(&mut paddle,&mut paddle_two, &mut ball);
        }
    }
}

