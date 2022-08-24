pub mod particle_emiter {

    use rand::Rng;
    use std::ops::{Index, IndexMut};

    pub const NUM_PARTICLES: usize = 1024;
    pub const WIDTH: i32 = 1280;
    pub const HEIGHT: i32 = 720;
    pub const PSIZE: i32 = 4;

    #[derive(Copy, Clone)]
    pub struct Particle {
        pub x: i32,
        pub y: i32,
        pub depth: u8,
    }

    pub struct Emiter {
        part: [Particle; NUM_PARTICLES],
    }

    impl Default for Particle {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Index<usize> for Emiter {
        type Output = Particle;
        fn index(&self, index: usize) -> &Self::Output {
            &self.part[index]
        }
    }
    
    impl IndexMut<usize> for Emiter {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.part[index]
        }
    }

    impl Particle {
        pub fn new() -> Self {
            let mut rng = rand::thread_rng();
            let rand_x = rng.gen_range(0..WIDTH - (PSIZE + 1));
            let rand_y = rng.gen_range(0..HEIGHT - (PSIZE + 1));
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
            if self.y > HEIGHT - (PSIZE + 1) {
                self.y = rng.gen_range(0..HEIGHT - (PSIZE + 1));
                self.x = rng.gen_range(0..WIDTH - (PSIZE + 1));
            }
        }
    }

    impl Default for Emiter {
        fn default() -> Self {
            Self::new()
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

        pub fn get_particle(&mut self, x: usize) -> Particle {
            return self.part[x];
        }
    }
}
