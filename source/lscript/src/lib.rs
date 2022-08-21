//! LScript - Movement of Pixel by instructions in text file (Script).
pub mod scr {

    /// Movement Direction
    #[derive(PartialEq, Copy, Clone, Debug)]
    pub enum Direction {
        Left,
        Right,
        Up,
        Down,
        Set,
    }

    /// Movement structure
    #[derive(Copy, Clone)]
    pub struct Movement {
        pub direction: Direction,
        pub steps: i32,
        pub pos: (i32, i32),
    }

    impl std::fmt::Display for Movement {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.direction == Direction::Set {
                return write!(f, "direction: {:?} pos: {},{}", self.direction, self.pos.0, self.pos.1)
            }
            write!(f, "direction: {:?} steps: {}", self.direction, self.steps)
        }
    }

    /// Container for Movement(s)
    pub struct MovementObject {
        lst: Vec<Movement>,
        index: usize,
    }

    impl MovementObject {
        /// static load from file function returns initalized MovementObject
        pub fn load_from_file(filename: &str) -> Self {
            let mut lst: Vec<Movement> = vec![];
            let contents = std::fs::read_to_string(filename).expect("Error reading the file");
            for i in contents.lines() {
                let trimmed = i.trim();
                let pos = trimmed.find(':');
                if pos == None {
                    continue;
                }
                let pos = pos.unwrap();
                let left = &trimmed[0..pos];
                let right = &trimmed[pos + 1..];
                let ch = left.chars().nth(0).unwrap();
                let mut p = (0, 0);
                let mut steps = 0;
                let dir: Direction = match ch {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'S' => Direction::Set,
                    _ => {
                        panic!("invalid type");
                    }
                };
                if dir == Direction::Set {
                    let cpos = right.find(",");
                    if cpos == None {
                        continue;
                    }
                    let le = &right[0..cpos.unwrap()];
                    let ri = &right[cpos.unwrap() + 1..];
                    p.0 = le.parse().unwrap();
                    p.1 = ri.parse().unwrap();
                } else {
                    steps = right.parse().unwrap();
                }
                let l: Movement = Movement {
                    direction: dir,
                    steps: steps,
                    pos: p,
                };
                lst.push(l);
            }
            MovementObject { lst: lst, index: 0 }
        }

        /// print the current movements
        pub fn print_movement(&self) {
            for i in &self.lst {
                println!("{}", i);
            }
        }

        /// get next position
        pub fn get_pos(&mut self) -> Movement {
            let m = self.lst.get(self.index).cloned();
            self.index += 1;
            if self.index >= self.lst.len() {
                self.index = 0;
            }
            m.unwrap()
        }
    }
}
