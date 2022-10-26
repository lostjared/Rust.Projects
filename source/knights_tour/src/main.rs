use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

fn main() {
    let width = 1280;
    let height = 720;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("App", width, height).resizable().opengl().build().unwrap();
    let mut can = window.into_canvas().build().map_err(|e| e.to_string()).expect("Error on canvas");
    let tc = can.texture_creator();
    let mut ksurf = Surface::load_bmp("knight.bmp").unwrap();
    ksurf.set_color_key(true, sdl2::pixels::Color::RGB(255, 255, 255)).expect("on color key");
    let knight = tc.create_texture_from_surface(ksurf).unwrap();   
    let mut e = sdl.event_pump().unwrap();

    let mut curcolor : sdl2::pixels::Color;
    let mut board : Box<[[u8; 8]; 8]> =  Box::new([[0; 8]; 8]); 
    let mut kx = 0;
    let mut ky = 0;


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
        can.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
        can.clear();

        

        let startx = 100;
	    let starty = 30;
	    let mut counter = 0;
	    let mut dx = startx;
	    let mut dy = starty;
        let mut ion = 1;

        for i in 0..8  {
            for z in 0..8 {
                if ion == 1 {
                    curcolor = sdl2::pixels::Color::RGB(255, 255, 255);
                }
                else {
                    curcolor = sdl2::pixels::Color::RGB(255, 0, 0);
                }
                ion = !ion;
                if board[i][z] == 0 {
                    can.set_draw_color(curcolor);
                    can.fill_rect(Some(Rect::new(dx, dy, 50, 50))).expect("on fill");
                }

                if kx == i && ky == z {
                    can.copy(
                        &knight,
                        None,
                        Some(Rect::new(
                           dx+5,
                            dy+5,
                            35,
                            35,
                        )),
                    )
                    .expect("copy tex");
                }

                dx += 55;
                counter += 1;
                if counter >= 8
                {
                    counter = 0;
                    dy += 55;
                    dx = startx;
                    ion = !ion;
                }
            }
        }


        can.present();
    }
}
