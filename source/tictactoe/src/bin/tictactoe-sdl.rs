use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use tictactoe::tictactoe::Grid;


fn main() {
    let row1 = vec![(250, 100), (650, 100), (1000, 100)];
    let row2 = vec![(250, 300), (650, 300), (1000, 300)];
    let row3 = vec![(250, 550), (650, 550), (1000, 550)];
    let cords = vec![row1, row2, row3];
    let mut game_over = false;
    let width = 1280;
    let height = 720;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("TicTacToe App", width, height)
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
    let font = ttf_context.load_font("./font.ttf", 18).expect("test");
    let _text_surf = font
            .render("Hello, World!")
            .blended(Color::RGB(255, 255, 255))
            .unwrap();

    let x_surf = font.render("X").blended(Color::RGB(255,255,255)).unwrap();
    let o_surf = font.render("O").blended(Color::RGB(255,255,255)).unwrap();
    let x_text = tc.create_texture_from_surface(&x_surf).unwrap();
    let o_text = tc.create_texture_from_surface(&o_surf).unwrap();

    let mut grid : Grid = Grid::new();
    let mut e = sdl.event_pump().unwrap();
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
                    game_over = false;
                    grid.clear();
                },
                Event::MouseButtonDown { x, y, .. } => {
                    if game_over == false {
                        grid.click(x, y)
                    }
                }
                _ => {

                }
            }
        }
        can.set_draw_color(Color::RGB(0, 0, 0));
        can.clear();

        can.set_draw_color(Color::RGB(255, 255, 255));
        can.fill_rect(Some(Rect::new(100, 200, 1280 - 200, 25)))
            .expect("draw rect");
        can.fill_rect(Some(Rect::new(100, 400, 1280 - 200, 25)))
            .expect("draw rect");
        can.fill_rect(Some(Rect::new(1280 / 2 - 200, 25, 25, 720 - 100)))
            .expect("draw rect");
        can.fill_rect(Some(Rect::new(1280 / 2 + 200, 25, 25, 720 - 100)))
            .expect("draw rect");

        for z in 0..3 {
            for i in 0..3 {
                let t = grid.grid[i][z];
                let x_pos = cords[z][i].0;
                let y_pos = cords[z][i].1;
                if t == 1 {
                    can.copy(&x_text, None, Some(Rect::new(x_pos, y_pos, 32, 32))).expect("on copy");
                } else if t == 2 {
                    can.copy(&o_text, None, Some(Rect::new(x_pos, y_pos, 32, 32))).expect("on copy");
                }
            }
        }

        if grid.check_game_over() != -1 {
            let game_over_surf = font.render(&format!("Game Over Player: {} Wins", grid.check_game_over())).blended(Color::RGB(255,255,255)).unwrap();
            let game_over_text = tc.create_texture_from_surface(&game_over_surf).unwrap();
            can.copy(&game_over_text, None, Some(Rect::new(25, 25, 200, 25))).expect("on copy");
            game_over = true;
        } 
        can.present();
    }
}
