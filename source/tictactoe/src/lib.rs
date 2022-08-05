pub mod tictactoe {

    pub struct Grid {
        pub grid: Box<[[u32; 4]; 4]>,
        pub turn: u32,
    }

    impl Default for Grid {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Grid {
        pub fn new() -> Self {
            Grid {
                grid: Box::new([[0; 4]; 4]),
                turn: 1,
            }
        }

        pub fn click(&mut self, x: i32, y: i32) {
            if x >= 100 && x <= 450 && y >= 30 && y <= 200 {
                self.set_turn(0, 0);
            } else if x >= 450 && x <= 850 && y >= 30 && y <= 200 {
                self.set_turn(1, 0);
            } else if x >= 800 && x <= 1100 && y >= 30 && y <= 200 {
                self.set_turn(2, 0);
            } else if x >= 100 && x <= 450 && y >= 250 && y <= 350 {
                self.set_turn(0, 1);
            } else if x >= 450 && x <= 850 && y >= 250 && y <= 350 {
                self.set_turn(1, 1);
            } else if x >= 800 && x <= 1100 && y >= 250 && y <= 350 {
                self.set_turn(2, 1);   
            } else if x >= 100 && x <= 450 && y > 400 && y <= 600 {
                self.set_turn(0, 2);
            } else if x >= 450 && x <= 850 && y > 400 && y <= 600 {
                self.set_turn(1, 2);
            } else if x >= 800 && x <= 1100 && y > 400 && y <= 600{
                self.set_turn(2,2);
            }
        }

        pub fn set_turn(&mut self, x: usize, y: usize) {
            if self.check_turn(x, y) {
                self.grid[x][y] = self.turn;
                self.switch_turn();
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
            if self.grid[0][1] == t && self.grid[1][1] == t && self.grid[2][1] == t {
                return true;
            }
            if self.grid[0][2] == t && self.grid[1][2] == t && self.grid[2][2] == t {
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
