
use console::console_system::Console;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

fn main() {
    let width = 1280;
    let height = 720;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    video.text_input().start();
    let window = video
        .window("Console", width, height)
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
    let mut flash = 0;
    let mut con = Console::new(25, 25, width as u32, height as u32);
    con.set_text_color(Color::RGB(255, 255, 255));
    con.print_prompt();
    con.set_visible(true);
    'main: loop {
        for _event in e.poll_iter() {
            match _event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::TextInput {
                    timestamp: _,
                    window_id: _,
                    text: s,
                } => {
                    con.type_key(&s);
                }
                Event::KeyDown { keycode: key, .. } => {
                    if key == Some(Keycode::Backspace) {
                        con.back();
                    }
                    if key == Some(Keycode::Return) {
                        if !con.get_visible() {
                            con.set_visible(true);
                        } else {
                            con.enter();
                        }
                    }
                }
                _ => {}
            }
        }
        can.set_draw_color(Color::RGB(0, 0, 0));
        can.clear();
        flash += 1;
        let flash_on;
        if flash > 10 {
            flash_on = true;
            flash = 0;
        } else {
            flash_on = false;
        }
        // con.print(&format!("hello world: {}\n", flash));
        con.draw(flash_on, &mut can, &tc, &font);
        can.present();
    }
}
