/// game module
pub mod game {

    use rand::Rng;

    /// puzzle Block information struct
    #[derive(Copy, Clone, Debug)]
    pub struct Block {
        pub x: i32,
        pub y: i32,
        pub color: i32,
    }

    pub const WINDOW_WIDTH: usize = 1280;
    pub const WINDOW_HEIGHT: usize = 720;
    pub const TILE_W: usize = WINDOW_WIDTH / 32;
    pub const TILE_H: usize = WINDOW_HEIGHT / 16;

    /// puzzle game Grid structure
    pub struct Grid {
        blocks: Box<[[Block; TILE_H]; TILE_W]>,
        width: i32,
        height: i32,
        piece: [Block; 3],
        pub score: u32,
        pub game_over: bool,
        piece_shape: i32,
        pub lines: i32,
    }

    impl Grid {
        /// create a new Grid object
        pub fn new(widthx: i32, heightx: i32) -> Grid {
            let g = Box::new(
                [[Block {
                    x: 0,
                    y: 0,
                    color: 0,
                }; TILE_H]; TILE_W],
            );
            Grid {
                blocks: g,
                width: widthx,
                height: heightx,
                piece: [Block {
                    x: 0,
                    y: 0,
                    color: 0,
                }; 3],
                score: 0,
                game_over: false,
                piece_shape: 0,
                lines: 0,
            }
        }

        /// create a new puzzle piece
        pub fn new_piece(&mut self) {
            let mut rng = rand::thread_rng();
            self.piece[0].x = (TILE_W / 2) as i32;
            self.piece[0].y = 0;
            self.piece[0].color = rng.gen_range(1..10);
            self.piece[1].x = (TILE_W / 2) as i32;
            self.piece[1].y = 1;
            self.piece[1].color = rng.gen_range(1..10);
            self.piece[2].x = (TILE_W / 2) as i32;
            self.piece[2].y = 2;
            self.piece[2].color = rng.gen_range(1..10);
            while self.piece[0].color == self.piece[1].color
                || self.piece[0].color == self.piece[2].color
            {
                self.piece[0].color = rng.gen_range(1..10);
                self.piece[1].color = rng.gen_range(1..10);
                self.piece[2].color = rng.gen_range(1..10);
            }
            self.piece_shape = 0;
        }

        /// reset the game
        pub fn reset_game(&mut self) {
            for x in 0..self.get_width() {
                for y in 0..self.get_height() {
                    self.blocks[x as usize][y as usize].color = 0;
                }
            }
            self.new_piece();
            self.score = 0;
            self.lines = 0;
        }

        /// swap the colors of the puzzle piece
        pub fn swap_piece_colors(&mut self, dir: u8) {
            let b = self.piece;
            if dir == 0 {
                self.piece[0].color = b[2].color;
                self.piece[1].color = b[0].color;
                self.piece[2].color = b[1].color;
            } else {
                self.piece[0].color = b[1].color;
                self.piece[1].color = b[2].color;
                self.piece[2].color = b[0].color;
            }
        }

        /// get grid information by point (x,y)
        pub fn get_grid_point(&self, x: usize, y: usize) -> i32 {
            self.blocks[x][y].color
        }

        /// get the currente game block
        pub fn get_block(&self) -> [Block; 3] {
            self.piece
        }

        /// get width of grid
        pub fn get_width(&self) -> i32 {
            self.width
        }

        /// get the height of grid
        pub fn get_height(&self) -> i32 {
            self.height
        }

        /// shift the game block left
        pub fn shift_left(&mut self) {
            if self.piece_shape == 0
                && self.check_block(0, self.piece[1].x - 1, self.piece[1].y - 1) 
                && self.check_block(0, self.piece[2].x - 2, self.piece[2].y - 2) 
            {
                self.piece[1].x -= 1;
                self.piece[1].y -= 1;
                self.piece[2].x -= 2;
                self.piece[2].y -= 2;
                self.piece_shape = 1;
            } else if self.piece_shape == 1
                && self.check_block(0, self.piece[1].x + 1, self.piece[1].y + 1) 
                && self.check_block(0, self.piece[2].x + 2, self.piece[2].y + 2) 
            {
                self.piece[1].x += 1;
                self.piece[1].y += 1;
                self.piece[2].x += 2;
                self.piece[2].y += 2;
                self.piece_shape = 0;
            }
        }

        /// shift the game block right
        pub fn shift_right(&mut self) {
            if self.piece_shape == 0
                && self.check_block(0, self.piece[1].x + 1, self.piece[1].y - 1) 
                && self.check_block(0, self.piece[2].x + 2, self.piece[2].y - 2) 
            {
                self.piece[1].x += 1;
                self.piece[1].y -= 1;
                self.piece[2].x += 2;
                self.piece[2].y -= 2;
                self.piece_shape = 2;
            } else if self.piece_shape == 2
                && self.check_block(0, self.piece[1].x + 1, self.piece[1].y + 1) 
                && self.check_block(0, self.piece[2].x + 2, self.piece[2].y + 2) 
            {
                self.piece[1].x -= 1;
                self.piece[1].y += 1;
                self.piece[2].x -= 2;
                self.piece[2].y += 2;
                self.piece_shape = 0;
            }
        }

        /// move the game block left
        pub fn move_left(&mut self) {
            let mut go = true;
            for i in 0..3 {
                if self.piece[i].x <= 0
                    || self.blocks[(self.piece[i].x as usize) - 1][self.piece[i].y as usize].color
                        != 0
                {
                    go = false;
                }
            }
            if go  {
                for i in 0..3 {
                    self.piece[i].x -= 1;
                }
            }
        }

        /// move the game block right
        pub fn move_right(&mut self) {
            let mut go = true;
            for i in 0..3 {
                if self.piece[i].x >= (TILE_W as i32) - 1
                    || self.blocks[(self.piece[i].x as usize) + 1][self.piece[i].y as usize].color
                        != 0
                {
                    go = false;
                }
            }
            if go  {
                for i in 0..3 {
                    self.piece[i].x += 1;
                }
            }
        }

        /// move the game block down
        pub fn move_down(&mut self) {
            if self.piece[2].y + 1 > (TILE_H as i32) - 1 {
                self.set_block();
                return;
            }

            if self.piece[2].y == 2
                && self.piece[2].y + 1 < (TILE_H as i32) - 1
                && self.blocks[self.piece[2].x as usize][(self.piece[2].y as usize) + 1].color != 0
            {
                self.reset_game();
                self.game_over = true;
                return;
            }

            for i in 0..3 {
                if self.piece[i].y + 1 < (TILE_H as i32)
                    && self.blocks[self.piece[i].x as usize][(self.piece[i].y as usize) + 1].color
                        != 0
                {
                    self.set_block();
                    return;
                }
            }

            for i in 0..3 {
                self.piece[i].y += 1;
            }
        }

        /// set the game block in the grid
        pub fn set_block(&mut self) {
            for i in 0..3 {
                self.blocks[self.piece[i].x as usize][self.piece[i].y as usize].color =
                    self.piece[i].color;
            }
            self.new_piece();
            self.proc_blocks();
        }

        /// check whether there is room for a block in the grid
        pub fn check_block(&mut self, color: i32, x: i32, y: i32) -> bool {
            if x >= 0
                && x < (TILE_W as i32)
                && y >= 0
                && y < (TILE_H as i32)
                && color == self.blocks[x as usize][y as usize].color
            {
                true
            } else {
                false
            }
        }

        /// process the block move down
        pub fn proc_move_down(&mut self) {
            for x in 0..self.get_width() {
                for y in 0..self.get_height() - 1 {
                    let color = self.blocks[x as usize][y as usize].color;
                    let color2 = self.blocks[x as usize][(y as usize) + 1].color;
                    if color != 0 && color2 == 0 {
                        self.blocks[x as usize][y as usize].color = 0;
                        self.blocks[x as usize][(y as usize) + 1].color = color;
                        return;
                    }
                }
            }
        }

        /// process the blocks
        pub fn proc_blocks(&mut self) {
            for x in 0..self.get_width() {
                for y in 0..self.get_height() {
                    let xpos: usize = x as usize;
                    let ypos: usize = y as usize;
                    let mut color: i32 = self.blocks[xpos][ypos].color;
                    if color >= 1 {
                        if self.check_block(color, x + 1, y) 
                            && self.check_block(color, x + 2, y) 
                        {
                            self.blocks[xpos][ypos].color = -1;
                            self.blocks[xpos + 1][ypos].color = -1;
                            self.blocks[xpos + 2][ypos].color = -1;

                            if self.check_block(color, x + 3, y)  {
                                self.blocks[xpos + 3][ypos].color = -1;
                                self.score += 1;
                                if self.check_block(color, x + 4, y)  {
                                    self.blocks[xpos + 4][ypos].color = -1;
                                    self.score += 1;
                                }
                            }
                            self.score += 1;
                            self.lines += 1;
                            return;
                        }
                        if self.check_block(color, x, y + 1) 
                            && self.check_block(color, x, y + 2) 
                        {
                            self.blocks[xpos][ypos].color = -1;
                            self.blocks[xpos][ypos + 1].color = -1;
                            self.blocks[xpos][ypos + 2].color = -1;

                            if self.check_block(color, x, y + 3)  {
                                self.blocks[xpos][ypos + 3].color = -1;
                                self.score += 1;
                                if self.check_block(color, x, y + 4)  {
                                    self.blocks[xpos][ypos + 4].color = -1;
                                    self.score += 1;
                                }
                            }

                            self.score += 1;
                            self.lines += 1;
                            return;
                        }
                        if self.check_block(color, x + 1, y + 1) 
                            && self.check_block(color, x + 2, y + 2) 
                        {
                            self.blocks[xpos][ypos].color = -1;
                            self.blocks[xpos + 1][ypos + 1].color = -1;
                            self.blocks[xpos + 2][ypos + 2].color = -1;

                            if self.check_block(color, x + 3, y + 3)  {
                                self.blocks[xpos + 3][ypos + 3].color = -1;
                                self.score += 2;
                                if self.check_block(color, x + 4, y + 4)  {
                                    self.blocks[xpos + 3][ypos + 3].color = -1;
                                    self.score += 2;
                                }
                            }

                            self.score += 2;
                            self.lines += 1;
                            return;
                        }
                        if self.check_block(color, x + 1, y - 1) 
                            && self.check_block(color, x + 2, y - 2) 
                        {
                            self.blocks[xpos][ypos].color = -1;
                            self.blocks[xpos + 1][ypos - 1].color = -1;
                            self.blocks[xpos + 2][ypos - 2].color = -1;

                            if self.check_block(color, x + 3, y - 3)  {
                                self.blocks[xpos + 3][ypos - 3].color = -1;
                                self.score += 2;
                                if self.check_block(color, x + 4, y - 4)  {
                                    self.blocks[xpos + 4][ypos - 4].color = -1;
                                    self.score += 2;
                                }
                            }
                            self.score += 2;
                            self.lines += 1;
                            return;
                        }
                        /*
                        if self.check_block(color, x - 1, y + 1) 
                            && self.check_block(color, x - 2, y + 2) 
                        {
                            self.blocks[xpos][ypos].color = -1;
                            self.blocks[xpos - 1][ypos + 1].color = -1;
                            self.blocks[xpos - 2][ypos + 2].color = -1;
                            self.score += 2;
                            return;
                        }*/
                    } else if color < 0 {
                        color -= 1;
                        if color < -90 {
                            self.blocks[xpos][ypos].color = 0;
                        } else {
                            self.blocks[xpos][ypos].color = color;
                        }
                    }
                }
            }
            self.proc_move_down();
        }
    }
}
