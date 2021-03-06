use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

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
    let text_surf_tex = tc.create_texture_from_surface(&text_surf).unwrap();
    let mut e = sdl.event_pump().unwrap();
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
        can.set_draw_color(Color::RGB(0, 0, 0));
        can.clear();
        let tc = can.texture_creator();
        let TextureQuery {
            width: wi,
            height: hi,
            ..
        } = text_surf_tex.query();
        can.copy(
            &text_surf_tex,
            Some(Rect::new(0, 0, wi, hi)),
            Some(Rect::new(25, 25, wi, hi)),
        )
        .expect("on font copy");
        can.present();
    }
}
