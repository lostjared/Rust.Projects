use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use std::time::{SystemTime, UNIX_EPOCH};

fn printtext(can: &mut sdl2::render::Canvas<sdl2::video::Window>, tex: &sdl2::render::TextureCreator<sdl2::video::WindowContext>, font: &sdl2::ttf::Font, x: i32, y: i32, color: sdl2::pixels::Color, text: &str) {
    let text_surf = font
    .render(text)
    .blended(color)
    .unwrap();
    let text_surf_tex = tex
    .create_texture_from_surface(&text_surf)
    .unwrap();
    let TextureQuery {
        width: wi,
        height: hi,
        ..
    } = text_surf_tex.query();
    can.copy(
        &text_surf_tex,
        Some(Rect::new(0, 0, wi, hi)),
        Some(Rect::new(x, y, wi, hi)),
    )
    .expect("on font copy");
}

fn main() {
    let width = 1280;
    let height = 720;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("App", width, height)
        .resizable()
        .opengl()
        .build()
        .unwrap();
    let mut can = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .expect("Error on canvas");
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let font = ttf_context.load_font("./font.ttf", 18).expect("test");
    let tc = can.texture_creator();
    let text_surf = font
        .render("Hello, World!")
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    let mut e = sdl.event_pump().unwrap();
    let mut prev_tick = 0;
    let mut ticks = 0;
    let mut frames = 0;
    let mut frame_count = 0;
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

        let start = SystemTime::now();
        let se = start.duration_since(UNIX_EPOCH).expect("error on time");
        let tick = se.as_secs() * 1000 + se.subsec_nanos() as u64 / 1_000_000;
        can.set_draw_color(Color::RGB(0, 0, 0));
        can.clear();
        ticks = tick / 1000;
        if ticks > prev_tick {
            prev_tick = ticks;
            frame_count += 1;
            frames = 0;
        } else {
            frames += 1;
        }

        let value = format!("time {}:{}:{}:{}", tick, ticks, frame_count, frames);
        printtext(&mut can, &tc, &font, 25, 25, Color::RGB(255, 255, 255), &value);
        can.present();
    }
}
