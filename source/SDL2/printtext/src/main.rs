use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;


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

fn printtext_width(can: &mut sdl2::render::Canvas<sdl2::video::Window>, tex: &sdl2::render::TextureCreator<sdl2::video::WindowContext>, font: &sdl2::ttf::Font, x: i32, y: i32, w: u32, color: sdl2::pixels::Color, text: &str) {

    let mut vlst : Vec<String>  = Vec::new();
    let mut width = x;
    let metrics = font.find_glyph_metrics('A').unwrap();
    //let mut ypos = y;

    let mut value = String::new();

    for ch in text.chars() {
        if (width + metrics.advance > (w-25) as i32) || ch == '\n' {
            vlst.push(value);
            value = String::new();
    //      ypos += metrics.advance+metrics.maxy;
            width = x;
        } else {
            value.push(ch);
            width += metrics.advance;
        }
    }
    vlst.push(value);

    let mut yy = y;
    for i in &vlst {
        if i.len() > 0 {
            printtext(can, tex, font, x, yy, color, i);
            yy += metrics.advance+metrics.maxy;
        }
    }

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
    let _text_surf = font
        .render("Hello, World!")
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
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
        printtext_width(&mut can, &tc, &font, 25, 25, width, Color::RGB(255, 255, 255), "Hello, World with SDL2/TTF!\nLine Two\nLine Three");
        can.present();
    }
}
