// cargo run --release
// followed this tutorial here: https://developer.mozilla.org/en-US/docs/Games/Techniques/Tilemaps
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::path::Path;
use sdl2::surface::Surface;
use std::time::{SystemTime, UNIX_EPOCH};

struct Camera {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    max_x: i32,
    max_y: i32,
    speed: i32
}

fn max(x: i32, x2: i32) -> i32 {
    if x > x2 {
        x
    } else {
        x2
    }
}

fn min(x: i32, x2: i32) -> i32 {
    if x < x2 {
        x
    } else {
        x2
    }
}

impl Camera {
    fn new(w: i32, h: i32, mx: i32, my: i32) -> Camera {
        Camera { 
            x:0,
            y:0, 
            width:w,
            height:h, 
            max_x: mx, 
            max_y: my,
            speed: 512
        }
    }
    fn move_camera(&mut self, delta: f64, dx: i32, dy: i32) {
        let dx_val : f64 = dx as f64 * self.speed as f64 * delta;
        let dy_val : f64 = dy as f64 * self.speed as f64 * delta;
        self.x += dx_val as i32;
        self.y += dy_val as i32;
        self.x = max(0, min(self.x, self.max_x));
        self.y = max(0, min(self.y, self.max_y));
    }
}

struct Map {
    cols: i32,
    rows: i32,
    tsize: i32,
    layers: Vec<u8>
}

impl Map {
    fn new(col: i32, row: i32, ts: i32, l: Vec<u8>) -> Map {
        Map {
            cols: col,
            rows: row,
            tsize: ts,
            layers: l
        }
    }

    fn draw_map(&self, texture: &sdl2::render::Texture, camera: &Camera, can: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let start_col =   camera.x / self.tsize as i32;
        let end_col    =  start_col + (camera.width / self.tsize as i32);
        let start_row  =  camera.y / self.tsize as i32;
        let end_row    =  start_row +  (camera.height / self.tsize as i32);
        let cx : i32 = camera.x as i32;
        let cy : i32 = camera.y as i32;
        let off_x : i32 = -cx + start_col as i32 * self.tsize as i32;
        let off_y : i32 = -cy + start_row as i32 * self.tsize as i32;
        for i in start_col ..= end_col {
            for z in start_row ..= end_row {
                let tile : u8 = self.layers[(z * self.rows + i) as usize];
                let x : i32 = (i- start_col) * self.tsize + off_x;
                let y : i32 = (z- start_row) * self.tsize + off_y;
                let t : i32 = tile as i32;
                if t != 0 {
                    can.copy(&texture, Some(Rect::new((t-1) as i32 * self.tsize as i32, 0, self.tsize as u32, self.tsize as u32)), Some(Rect::new(x as i32, y as i32, self.tsize as u32, self.tsize as u32))).expect("on copy");
                }
            }
        }
    }
}

fn main() {
    let pathval = Path::new("./image.bmp");
    let width = 512;
    let height = 512;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("Tilemap App", width, height).resizable().opengl().build().unwrap();
    let mut can = window.into_canvas().build().map_err(|e| e.to_string()).expect("Error on canvas");
    let tc = can.texture_creator();
    let surf = Surface::load_bmp(pathval).unwrap();
    let texture = tc.create_texture_from_surface(surf).unwrap();
    let mut e = sdl.event_pump().unwrap();
    let tiles = vec![
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3,
        3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3,
        3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3,
        3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3,
        3, 1, 1, 1, 1, 1, 2, 1, 1, 1, 3, 3,
        3, 1, 2, 2, 1, 1, 1, 1, 1, 1, 3, 3,
        3, 1, 2, 2, 1, 1, 1, 1, 1, 1, 3, 3,
        3, 1, 1, 1, 2, 1, 1, 1, 1, 1, 3, 3,
        3, 1, 1, 1, 1, 2, 1, 1, 1, 1, 3, 3,
        3, 1, 1, 1, 1, 2, 1, 1, 1, 1, 3, 3,
        3, 3, 3, 1, 1, 2, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 1, 1, 2, 3, 3, 3, 3, 3, 3,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];
    let map = Map::new(12, 12, 64, tiles);
    let max_x : i32 = map.cols * 64 - 512;
    let max_y : i32 = map.cols * 64 - 512;
    let mut camera = Camera::new(512, 512, max_x, max_y);
    let mut prev_tick : u64 = 0;
    
    'main: loop {
        let start = SystemTime::now();
        let se = start.duration_since(UNIX_EPOCH).expect("error on time");
        let tick = se.as_secs() * 1000 + se.subsec_nanos() as u64 / 1_000_000;
        let mut delta : f64 = (tick as f64 - prev_tick as f64) / 1000.0;
        if delta > 0.15 {
            delta = 0.15;
        }
        prev_tick = tick; 
        for _event in e.poll_iter() {
            match _event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    camera.move_camera(delta, 1, 0)
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    camera.move_camera(delta, -1, 0);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    camera.move_camera(delta, 0, -1);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    camera.move_camera(delta, 0, 1);
                },
                _ => {}
            }
        }
        can.clear();
        map.draw_map(&texture, &camera, &mut can);
        can.present();
    }
}