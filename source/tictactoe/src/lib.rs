pub mod tictactoe {

    pub struct Grid {
        grid: Box<[[u32; 4]; 4]>,
        pub turn: u32,
    }

    impl Default for Grid {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Grid {
        pub fn new() -> Grid {
            Grid {
                grid: Box::new([[0; 4]; 4]),
                turn: 1,
            }
        }

        pub fn go_turn(&mut self, x: usize, y: usize, t: u32) {
            self.grid[x][y] = t;
        }

        pub fn check_type(&self, t: u32) -> bool {
            if self.grid[0][0] == t && self.grid[0][1] == t && self.grid[0][2] == t {
                return true;
            }
            if self.grid[1][0] == t && self.grid[1][1] == t && self.grid[1][2] == t {
                return true;
            }
            if self.grid[2][0] == t && self.grid[2][1] == t && self.grid[2][2] == t {
                return true;
            }

            if self.grid[0][0] == t && self.grid[0][1] == t && self.grid[0][2] == t {
                return true;
            }

            if self.grid[1][0] == t && self.grid[1][1] == t && self.grid[1][2] == t {
                return true;
            }
            if self.grid[2][0] == t && self.grid[2][1] == t && self.grid[2][2] == t {
                return true;
            }
            if self.grid[0][0] == t && self.grid[1][1] == t && self.grid[2][2] == t {
                return true;
            }
            if self.grid[0][2] == t && self.grid[1][1] == t && self.grid[2][0] == t {
                return true;
            }
            false
        }

        pub fn check_game_over(&self) -> i32 {
            if self.check_type(1) {
                return 1;
            }

            if self.check_type(2) {
                return 2;
            }

            -1
        }

        pub fn check_turn(&self, x: usize, y: usize) -> bool {
            if self.grid[x][y] == 0 {
                return true;
            }
            false
        }

        pub fn switch_turn(&mut self) {
            if self.turn == 2 {
                self.turn = 1;
            } else {
                self.turn = 2;
            }
        }

        pub fn print_board(&self) {
            for z in 0..3 {
                print!("|");
                for i in 0..3 {
                    if self.grid[i][z] == 1 {
                        print!("X");
                    } else if self.grid[i][z] == 2 {
                        print!("O");
                    } else {
                        print!("_");
                    }
                }
                println!("|");
            }
        }
    }
}
