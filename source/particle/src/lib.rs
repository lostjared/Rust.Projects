pub mod particle_emiter {

    use rand::Rng;

    pub const NUM_PARTICLES: usize = 1024;

    #[derive(Copy, Clone)]
    pub struct Particle {
        pub x: i32,
        pub y: i32,
        pub depth: u8,
    }

    pub struct Emiter {
        pub part: [Particle; NUM_PARTICLES],
    }

    impl Particle {
        pub fn new() -> Self {
            let mut rng = rand::thread_rng();
            let rand_x = rng.gen_range(0..1280 - 32);
            let rand_y = rng.gen_range(0..720 - 32);
            let rand_depth = rng.gen_range(0..255 - 16);
            Particle {
                x: rand_x,
                y: rand_y,
                depth: rand_depth,
            }
        }
        pub fn move_down(&mut self) {
            let mut rng = rand::thread_rng();
            self.y += 16;
            self.depth += 16;
            if self.depth >= 0xFE - 16 {
                self.depth = 1;
            }
            if self.y > 720 - 8 {
                self.y = rng.gen_range(0..720 - 32);
                self.x = rng.gen_range(0..1280 - 32);
            }
        }
    }

    impl Emiter {
        pub fn new() -> Self {
            Emiter {
                part: [Particle {
                    x: 0,
                    y: 0,
                    depth: 0,
                }; NUM_PARTICLES],
            }
        }

        pub fn init(&mut self) {
            for i in 0..NUM_PARTICLES {
                self.part[i] = Particle::new();
            }
        }

        pub fn update(&mut self) {
            for i in &mut self.part {
                i.move_down();
            }
        }
    }
}
