

pub mod game {

    #[derive(Copy,Clone,Debug)]
    pub struct Block {
        x: i32,
        y: i32,
        color: i32,
    }

    const TILE_W: usize = 1920/32;
    const TILE_H: usize = 1080/16;

    pub struct Grid {
        blocks: Box<[[Block; TILE_H]; TILE_W]>,
        width: i32,
        height: i32,
    }

    impl Block {
        fn new(xx: i32, xy: i32, xcolor: i32) -> Block {
            Block {x: xx, y: xy, color: xcolor}
        }
    }

    impl Grid {
        pub fn new(widthx: i32, heightx: i32) -> Grid {
            let g = Box::new([[Block{x: 0, y: 0, color: 0}; TILE_H]; TILE_W]);
            Grid{ blocks: g, width: widthx, height: heightx }
        }

        pub fn get_grid_point(&self, x: usize, y: usize) -> i32 {
            self.blocks[x][y].color
        }

        pub fn get_width(&self) -> i32 {
            self.width
        }

        pub fn get_height(&self) -> i32 {
            self.height
        }
    }

}