pub mod level {

    use sdl2::rect::Rect;
    use std::cmp::max;
    use std::cmp::min;

    pub struct Camera {
        pub x: i32,
        pub y: i32,
        pub width: i32,
        pub height: i32,
        pub max_x: i32,
        pub max_y: i32,
        pub speed: i32,
    }

    impl Camera {
        pub fn new(w: i32, h: i32, mx: i32, my: i32) -> Camera {
            Camera {
                x: 0,
                y: 0,
                width: w,
                height: h,
                max_x: mx,
                max_y: my,
                speed: 512,
            }
        }
        pub fn move_camera(&mut self, delta: f64, dx: i32, dy: i32) {
            let dx_val: f64 = dx as f64 * self.speed as f64 * delta;
            let dy_val: f64 = dy as f64 * self.speed as f64 * delta;
            self.x += dx_val as i32;
            self.y += dy_val as i32;
            self.x = max(0, min(self.x, self.max_x));
            self.y = max(0, min(self.y, self.max_y));
        }
    }

    pub struct Map {
        pub cols: i32,
        pub rows: i32,
        pub tsize: i32,
        pub layers: Vec<u8>,
    }

    impl Map {
        pub fn new(col: i32, row: i32, ts: i32, l: Vec<u8>) -> Map {
            Map {
                cols: col,
                rows: row,
                tsize: ts,
                layers: l,
            }
        }

        pub fn draw_map(
            &self,
            texture: &sdl2::render::Texture,
            camera: &Camera,
            can: &mut sdl2::render::Canvas<sdl2::video::Window>,
        ) {
            let start_col = camera.x / self.tsize as i32;
            let end_col = start_col + (camera.width / self.tsize as i32);
            let start_row = camera.y / self.tsize as i32;
            let end_row = start_row + (camera.height / self.tsize as i32);
            let cx: i32 = camera.x as i32;
            let cy: i32 = camera.y as i32;
            let off_x: i32 = -cx + start_col as i32 * self.tsize as i32;
            let off_y: i32 = -cy + start_row as i32 * self.tsize as i32;
            for i in start_col..=end_col {
                for z in start_row..=end_row {
                    let tile: u8 = self.layers[(z * self.rows + i) as usize];
                    let x: i32 = (i - start_col) * self.tsize + off_x;
                    let y: i32 = (z - start_row) * self.tsize + off_y;
                    let t: i32 = tile as i32;
                    if t != 0 {
                        can.copy(
                            texture,
                            Some(Rect::new(
                                (t - 1) as i32 * self.tsize as i32,
                                0,
                                self.tsize as u32,
                                self.tsize as u32,
                            )),
                            Some(Rect::new(
                                x as i32,
                                y as i32,
                                self.tsize as u32,
                                self.tsize as u32,
                            )),
                        )
                        .expect("on copy");
                    }
                }
            }
        }
    }
}
