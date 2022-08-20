//! LScript - Movement of Pixel by instructions in text file (Script).
pub mod scr {

    /// Movement Direction
    #[derive(PartialEq,Copy,Clone,Debug)]
    pub enum Direction {
        Left,
        Right,
        Up,
        Down
    }

    /// Movement structure
    #[derive(Copy, Clone)]
    pub struct Movement {
        pub direction: Direction,
        pub steps: i32
    }

    /// Container for Movement(s)
    pub struct MovementObject {
        lst: Vec<Movement>,
        index: usize,
    }

    impl MovementObject {

        /// static load from file function returns initalized MovementObject
        pub fn load_from_file(filename: &str) -> Self {
            let mut lst : Vec<Movement> = vec![];
            let contents = std::fs::read_to_string(filename).expect("Error reading the file");
            for i in contents.lines() {
                let pos = i.find(':');
                if pos == None {
                    continue;
                }
                let pos = pos.unwrap();
                let left = &i[0..pos];
                let right = &i[pos+1..];
                let ch = left.chars().nth(0).unwrap();
                let dir : Direction = match ch {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    _ => { panic!("invalid type"); }
                };
                let l : Movement = Movement { direction: dir, steps: right.parse().unwrap() };
                lst.push(l);
            }
            MovementObject {
                lst: lst,
                index: 0
            }
        }

        /// print the current movements
        pub fn print_movement(&self) {
            for i in &self.lst {
                println!("Move: {:?}, Steps: {}", i.direction, i.steps);
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