

pub mod game {

    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use rand::Rng;

    #[derive(Copy, Clone, Debug)]
    pub enum Movement {
        Left,
        Right,
    }

    #[derive(Copy, Clone, Debug)]
    struct Ball {
        pub x: i32,
        pub y: i32,
        pub col: (u8, u8, u8),
        pub speed: i32,
    }

    struct Emiter {
        pub particles: Vec<Ball>,
    }

    #[derive(Copy, Clone, Debug)]
    struct Glove {
        pub x: i32,
        pub y: i32,
    }

    pub struct Game {
        emiter: Emiter,
        glove: Glove,
        width: i32,
        height: i32,
    }

    impl Ball {
        pub fn new() -> Self {
            Ball {
                x: 0,
                y: 0,
                col: (0,0,0),
                speed: 0,
            }
        }

        pub fn gen_release() -> Self {
            let ball_y = 0;
            let mut r = rand::thread_rng();
            let ball_x = r.gen_range(0..1280-32);
            Ball {
                x: ball_x,
                y: ball_y,
                col: (255, 255, 255),
                speed: 10
            }
        }
    }
    
    impl Emiter {
        pub fn new() -> Self {
            Emiter {
                particles: Vec::new(),
            }
        }

        pub fn release(&mut self) {
            let b = Ball::gen_release();
            self.particles.push(b);
        }
    }

    impl Glove {
        pub fn new(glove_x: i32, glove_y: i32) -> Self {
            Glove {
                x: glove_x,
                y: glove_y,
            }
        }
    }

    impl Game {
        pub fn new(widthx: i32, heightx: i32) -> Self {
            Game {
                emiter: Emiter::new(),
                glove: Glove::new((widthx/2)-50, heightx-100),
                width: widthx,
                height: heightx,
            }
        }

        pub fn new_game(&mut self) {
            self.emiter.release();
        }

        pub fn draw(&mut self, can: &mut sdl2::render::Canvas<sdl2::video::Window>) {
            can.set_draw_color(Color::RGB(255, 255, 255));
            can.fill_rect(Some(Rect::new(self.glove.x, self.glove.y, 100, 100))).expect("on fill");
            for i in &self.emiter.particles {
                can.set_draw_color(Color::RGB(i.col.0, i.col.1, i.col.2));
                can.fill_rect(Some(Rect::new(i.x, i.y, 32, 32))).expect("on fill");
            }
        }

        pub fn logic(&mut self) {
            for i in 0..self.emiter.particles.len() {
                if self.emiter.particles[i].y < self.height-32 {
                    self.emiter.particles[i].y += 16;
                } else {
                    self.emiter.particles[i] = Ball::gen_release();
                }
                let p = self.emiter.particles[i];

                let r = sdl2::rect::Rect::new(self.glove.x-50, self.glove.y, 150, 100);
                let po = sdl2::rect::Point::new(self.emiter.particles[i].x, self.emiter.particles[i].y);
                if r.contains_point(po) {
                    self.emiter.particles[i] = Ball::gen_release();
                    // score += 1;
                    //self.emiter.release();
                    println!("HERE!");
                }
            }
        }
        pub fn keypress(&mut self, movement: Movement) {
            match movement {
                Movement::Left => {
                    if self.glove.x > 20 {
                        self.glove.x -= 20;
                    }
                }
                Movement::Right => {
                    if self.glove.x < self.width - 120 {
                        self.glove.x += 20;
                    }
                }
            }
        }
    }

}

