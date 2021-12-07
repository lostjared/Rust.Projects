

pub mod game {

    use rand::Rng;

    #[derive(Copy,Clone,Debug)]
    pub struct Block {
        pub x: i32,
        pub y: i32,
        pub color: i32,
    }

    pub const WINDOW_WIDTH : usize = 1280;
    pub const WINDOW_HEIGHT: usize = 720;
    pub const TILE_W: usize = WINDOW_WIDTH/32;
    pub const TILE_H: usize = WINDOW_HEIGHT/16;

    pub struct Grid {
        blocks: Box<[[Block; TILE_H]; TILE_W]>,
        width: i32,
        height: i32,
        piece: [Block; 3]
    }

    impl Grid {
        pub fn new(widthx: i32, heightx: i32) -> Grid {
            let g = Box::new([[Block{x: 0, y: 0, color: 0}; TILE_H]; TILE_W]);
            Grid{ blocks: g, width: widthx, height: heightx, piece: [Block { x: 0, y: 0, color: 0}; 3] }
        }

        pub fn new_piece(&mut self) {
            let mut rng = rand::thread_rng();
            self.piece[0].x = (TILE_W/2) as i32;
            self.piece[0].y = 0;
            self.piece[0].color = rng.gen_range(1..8);
            self.piece[1].x = (TILE_W/2) as i32;
            self.piece[1].y = 1;
            self.piece[1].color = rng.gen_range(1..8);
            self.piece[2].x = (TILE_W/2) as i32;
            self.piece[2].y = 2;
            self.piece[2].color = rng.gen_range(1..8);
            while self.piece[0].color == self.piece[1].color || self.piece[0].color == self.piece[2].color {
                self.piece[0].color = rng.gen_range(1..8);
                self.piece[1].color = rng.gen_range(1..8);
                self.piece[2].color = rng.gen_range(1..8);
            }
        }

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

        pub fn get_grid_point(&self, x: usize, y: usize) -> i32 {
            self.blocks[x][y].color
        }

        pub fn get_block(&self) -> [Block; 3] {
            self.piece
        }

        pub fn get_width(&self) -> i32 {
            self.width
        }

        pub fn get_height(&self) -> i32 {
            self.height
        }
        
        pub fn move_left(&mut self) {
            let mut go = true;
            for i in 0..3 {
                if self.piece[i].x <= 0 || self.blocks[(self.piece[i].x as usize)-1][self.piece[i].y as usize].color != 0 {
                    go = false;
                }
            }
            if go == true {
                for i in 0..3 {
                    self.piece[i].x -= 1;
                }
            }
        }

        pub fn move_right(&mut self) {
            let mut go = true;
            for i in 0..3 {
                if self.piece[i].x >= (TILE_W as i32)-1 || self.blocks[(self.piece[i].x as usize)+1][self.piece[i].y as usize].color != 0 {
                    go = false;
                }
            }
            if go == true {
                for i in 0..3 {
                    self.piece[i].x += 1;
                }
            }
        }

        pub fn move_down(&mut self) {
            if self.piece[2].y+1 > (TILE_H as i32)-1 {
                self.set_block();
                return;
            }
            if self.piece[2].y+1 < (TILE_H as i32)-1 && self.blocks[self.piece[2].x as usize][(self.piece[2].y as usize)+1].color != 0 {
                self.set_block();
                return;
            }

           for i in 0..3 {
                self.piece[i].y += 1;
            }
        }

        pub fn set_block(&mut self) {
            for i in 0..3 {
                self.blocks[self.piece[i].x as usize][self.piece[i].y as usize].color = self.piece[i].color;
            }           
            self.new_piece();
            self.proc_blocks();
        }

        pub fn check_block(&mut self, color: i32, x: i32, y: i32) -> bool {
            println!("true; x:{} y:{} color:{}", x,y,color);
            if x >= 0 && x < (TILE_W as i32) && y >= 0 && y < (TILE_H as i32) && color == self.blocks[x as usize][y as usize].color {
                return true;
            } else {
                return false;
            }
        }

        pub fn proc_move_down(&mut self) {
            for x in 0..self.get_width() {
                for y in 0..self.get_height()-1 {
                    let color = self.blocks[x as usize][y as usize].color;
                    let color2 = self.blocks[x as usize][(y as usize)+1].color;
                    if color != 0 && color2 == 0 {
                        self.blocks[x as usize][y as usize].color = 0;
                        self.blocks[x as usize][(y as usize)+1].color = color;
                        return;
                    }
                }
            }
        }

        pub fn proc_blocks(&mut self) {           
             for x in 0..self.get_width() {
                for y in 0..self.get_height() {
                    let xpos : usize = x as usize;
                    let ypos : usize = y as usize;
                    let mut color : i32 = self.blocks[xpos][ypos].color;
                    if color >= 1 {
                        if self.check_block(color, x+1, y) == true && self.check_block(color, x+2, y) == true {
                            self.blocks[xpos][ypos].color = -1;
                            self.blocks[xpos+1][ypos].color = -1;
                            self.blocks[xpos+2][ypos].color = -1;
                            return;
                        } 
                        if self.check_block(color, x, y+1) == true && self.check_block(color, x, y+2) == true {
                            self.blocks[xpos][ypos].color = -1;
                            self.blocks[xpos][ypos+1].color = -1;
                            self.blocks[xpos][ypos+2].color = -1;
                            return;
                        }
                        if self.check_block(color, x+1, y+1) == true && self.check_block(color, x+2, y+2) == true {
                            self.blocks[xpos][ypos].color = -1;
                            self.blocks[xpos+1][ypos+1].color = -1;
                            self.blocks[xpos+2][ypos+2].color = -1;
                            return;
                        }
                        if self.check_block(color, x+1, y-1) == true && self.check_block(color, x+2, y-2) == true {
                            self.blocks[xpos][ypos].color = -1;
                            self.blocks[xpos+1][ypos-1].color = -1;
                            self.blocks[xpos+2][ypos-2].color = -1;
                            return;
                        }
                        if self.check_block(color, x-1, y+1) == true && self.check_block(color, x-2, y+2) == true {
                            self.blocks[xpos][ypos].color = -1;
                            self.blocks[xpos-1][ypos+1].color = -1;
                            self.blocks[xpos-2][ypos+2].color = -1;
                            return;
                        }
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