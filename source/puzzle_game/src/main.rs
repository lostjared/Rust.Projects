
// cargo run --release

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use rand::Rng;
mod puzzle;
use puzzle::game;

fn draw_grid(grid : &game::Grid, colors: &Vec<Color>, can: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let mut rng = rand::thread_rng();
    let offset = 10;
    for x in 0..grid.get_width() as usize {
        for y in 0..grid.get_height() as usize {
            let mut color = grid.get_grid_point(x, y);
            color = rng.gen_range(0..7); // for now
            let value: Color = *colors.get(color as usize).unwrap();
            can.set_draw_color(value);
            can.fill_rect(Some(Rect::new(x as i32 * 32, (y as i32 * 16) + offset, 32, 16))).expect("draw rect");
        }
    }
}

fn main() {   
    let width = 1920;
    let height = 1080;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("Generic Puzzle Game", width, height).resizable().opengl().build().unwrap();
    let mut can = window.into_canvas().build().map_err(|e| e.to_string()).expect("Error on canvas");
    let tc = can.texture_creator();
    let mut texture = tc.create_texture_streaming(PixelFormatEnum::RGB24, width, height).map_err(|e| e.to_string()).expect("Error on texture create");
    let mut e = sdl.event_pump().unwrap();
    can.set_draw_color(Color::RGB(0, 0, 0));
    can.clear();
    can.present();
    let grid : game::Grid = game::Grid::new(1920/32, 1080/16);
    let mut colors = vec![];
    colors.push(Color::RGB(255, 0, 0));
    colors.push(Color::RGB(0, 255, 0));
    colors.push(Color::RGB(0, 0, 255));
    colors.push(Color::RGB(255, 255, 0));
    colors.push(Color::RGB(0, 255, 255));
    colors.push(Color::RGB(255, 255, 255));
    colors.push(Color::RGB(255, 0, 255,));
    colors.push(Color::RGB(150, 155, 40));
    colors.push(Color::RGB(50, 150, 200));
    'main: loop {
        for _event in e.poll_iter() {
            match _event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
           for y in 0..height {
                for x in 0..width {
                    let offset = y  * pitch as u32 + x * 3;
                    buffer[offset as usize] = 0;
                    buffer[offset as usize + 1] = 0;
                    buffer[offset as usize + 2] = 0;
                }
            }
        }).expect("on lock");
        can.set_draw_color(Color::RGB(0, 0, 0));
        can.clear();
        draw_grid(&grid,&colors,&mut can);
        //   can.copy(&texture, None, Some(Rect::new(0, 0, width, height))).expect("on copy");
        can.present();
    }
}