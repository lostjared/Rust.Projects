pub mod rmx_system {

    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;

    pub struct System {
        sdl: sdl2::Sdl,
        can: sdl2::render::Canvas<sdl2::video::Window>,
        pub screen: usize,
    }

    pub trait ScreenTrait {
        fn draw(&mut self, can: &mut sdl2::render::Canvas<sdl2::video::Window>);
        fn keydown(&mut self, k: sdl2::keyboard::Keycode);
        fn keyup(&mut self, k: sdl2::keyboard::Keycode);
    }

    impl System {
        pub fn init(title: &str, width: u32, height: u32) -> System {
            let sdl = sdl2::init().unwrap();
            let video = sdl.video().unwrap();
            video.text_input().start();
            let window = video
                .window(title, width, height)
                .resizable()
                .opengl()
                .build()
                .unwrap();
            let can = window
                .into_canvas()
                .build()
                .map_err(|e| e.to_string())
                .expect("Error on canvas");

            System { sdl: sdl, can: can, screen: 0 }
        }

        pub fn set_screen(&mut self, i: usize) {
            self.screen = i;
        }

        pub fn get_screen(&mut self) -> usize {
            self.screen
        }

        pub fn exec<T: ScreenTrait>(&mut self, obj: &mut T) {
            let mut e = self.sdl.event_pump().unwrap();
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
                            text: _s,
                        } => {
                           
                        }
                        Event::KeyDown { keycode: key, .. } => {
                            obj.keydown(key.unwrap());
                        }
                        Event::KeyUp { keycode: key, .. } => {
                            obj.keyup(key.unwrap());
                        }
                        _ => {}
                    }
                }
                self.can.set_draw_color(Color::RGB(0, 0, 0));
                obj.draw(&mut self.can);
                self.can.present();
            }
        }
    }
}
