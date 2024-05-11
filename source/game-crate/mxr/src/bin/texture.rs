// draw to offscreen 640x480 surface
// copy surface to screen as screen as 1440x1080 or width height as arguments

use mxr::mxr::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String> {
    let mut width = 1440;
    let mut height = 1080;
    let args : Vec<String> = std::env::args().collect();
    if args.len() == 3 {
        width = args[1].parse::<u32>().unwrap();
        height = args[2].parse::<u32>().unwrap();
    }
    let mut mx = MXWindowBuilder::new()
        .create("Hello World [load_gfx]", width, height)
        .build()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("./data/font.ttf", 18)?;
    let tc = mx.can.texture_creator();
    let files = vec!["./data/logo.bmp"];
    let textures = mx.load_gfx(files, &tc, None)?;
    let tex = mx
        .printtext_texture(
            &font,
            &tc,
            sdl2::pixels::Color::RGB(255, 255, 255),
            &format!("print text to texture test"),
        )
        .unwrap();

    let tex_s = tex_get_size(&tex);
    let mut texture = tc
    .create_texture_target(tc.default_pixel_format(), 640, 480)
    .unwrap();
    'main: loop {
        for event in mx.event.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }
        mx.can.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            texture_canvas.clear();  
            texture_canvas
            .copy(&textures[0], None, None)
            .expect("Failure to copy texture to canvas");
            texture_canvas
            .copy(&tex, None, sdl2::rect::Rect::new(15, 15, tex_s.0, tex_s.1))
            .expect("on copy");
        }).map_err(|x| x.to_string())?;
        mx.can.clear();
        mx.can.copy(&texture, None, None)?;
        mx.can.present();
    }
    Ok(())
}
