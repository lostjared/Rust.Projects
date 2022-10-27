use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::surface::Surface;

fn main() {
    let width = 1280;
    let height = 720;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window(
            "Knights Tour - [Press Space to move, Return to Reset]",
            width,
            height,
        )
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
    let tc = can.texture_creator();
    let font = ttf_context
        .load_font("./font.ttf", 18)
        .expect("error loading font");
    let _text_surf = font
        .render("Hello, World!")
        .blended(sdl2::pixels::Color::RGB(255, 255, 255))
        .unwrap();

    let mut ksurf = Surface::load_bmp("knight.bmp").unwrap();
    ksurf
        .set_color_key(true, sdl2::pixels::Color::RGB(255, 255, 255))
        .expect("on color key");
    let knight = tc.create_texture_from_surface(ksurf).unwrap();
    let mut e = sdl.event_pump().unwrap();

    let mut curcolor: sdl2::pixels::Color;
    let mut board: Box<[[u8; 8]; 8]> = Box::new([[0; 8]; 8]);
    let mut rowx: i32 = 1;
    let mut colx: i32 = 6;
    let mut moves = 0;
    let htable = [
        [2, 3, 4, 4, 4, 4, 3, 2],
        [3, 4, 6, 6, 6, 6, 4, 3],
        [4, 6, 8, 8, 8, 8, 6, 4],
        [4, 6, 8, 8, 8, 8, 6, 4],
        [4, 6, 8, 8, 8, 8, 6, 4],
        [4, 6, 8, 8, 8, 8, 6, 4],
        [3, 4, 6, 6, 6, 6, 4, 3],
        [2, 3, 4, 4, 4, 4, 3, 2],
    ];

    let horizontal = [2, 1, -1, -2, -2, -1, 1, 2];
    let vertical = [-1, -2, -2, -1, 1, 2, 2, 1];

    'main: loop {
        for _event in e.poll_iter() {
            match _event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    for i in 0..8 {
                        for z in 0..8 {
                            board[i][z] = 0;
                        }
                    }

                    rowx = 1;
                    colx = 6;
                    moves = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    let mut smallest = 100;
                    let mut choice: i32 = -1;

                    for i in 0..8 {
                        let mut row;
                        let mut col;
                        row = rowx;
                        col = colx;
                        row += horizontal[i];
                        col += vertical[i];
                        if row >= 0
                            && row < 8
                            && col >= 0
                            && col < 8
                            && board[row as usize][col as usize] == 0
                        {
                            if htable[row as usize][col as usize] < smallest
                                && htable[row as usize][col as usize] != 0
                            {
                                smallest = htable[row as usize][col as usize];
                                choice = i as i32;
                            }
                        }
                    }

                    if choice != -1 {
                        board[rowx as usize][colx as usize] = 1;
                        rowx += horizontal[choice as usize] as i32;
                        colx += vertical[choice as usize] as i32;
                        moves += 1;
                        if moves >= 63 {
                            moves += 1;
                            board[rowx as usize][colx as usize] = 1;
                        }
                    }
                }
                _ => {}
            }
        }
        can.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        can.clear();

        let startx = 100;
        let starty = 30;
        let mut counter = 0;
        let mut dx = startx;
        let mut dy = starty;
        let mut ion = 1;

        for i in 0..8 {
            for z in 0..8 {
                if ion == 1 {
                    curcolor = sdl2::pixels::Color::RGB(255, 255, 255);
                } else {
                    curcolor = sdl2::pixels::Color::RGB(255, 0, 0);
                }
                ion = !ion;
                if board[i][z] == 0 {
                    can.set_draw_color(curcolor);
                    can.fill_rect(Some(Rect::new(dx, dy, 50, 50)))
                        .expect("on fill");
                }

                if rowx == i as i32 && colx == z as i32 {
                    can.copy(&knight, None, Some(Rect::new(dx + 5, dy + 5, 35, 35)))
                        .expect("copy tex");
                }

                dx += 55;
                counter += 1;
                if counter >= 8 {
                    counter = 0;
                    dy += 55;
                    dx = startx;
                    ion = !ion;
                }
            }
        }

        let menu_string = format!("Knights Tour - Moves: {}", moves);

        let turn_surf = font
            .render(&format!("{}", menu_string))
            .blended(sdl2::pixels::Color::RGB(255, 255, 255))
            .unwrap();
        let turn_surf_text = tc.create_texture_from_surface(&turn_surf).unwrap();

        let TextureQuery {
            width: wi,
            height: hi,
            ..
        } = turn_surf_text.query();

        can.copy(&turn_surf_text, None, Some(Rect::new(600, 250, wi, hi)))
            .expect("on copy");

        can.present();
    }
}
