pub mod rmx_console;

pub mod rmx_system {

    use crate::rmx_console::Console;
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;

    pub struct System {
        pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
        pub screen: usize,
        e: sdl2::EventPump,
        pub console: Console,
    }

    pub trait ScreenTrait {
        fn draw(&mut self, scr: usize, system: &mut System) -> usize;
        fn keydown(&mut self, scr: usize, k: sdl2::keyboard::Keycode) -> usize;
        fn keyup(&mut self, scr: usize, k: sdl2::keyboard::Keycode) -> usize;
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

            let ev = sdl.event_pump().unwrap();

            System {
                canvas: can,
                screen: 0,
                e: ev,
                console: Console::new(20, 20, width - 20, height - 20),
            }
        }

        pub fn set_screen(&mut self, i: usize) {
            self.screen = i;
        }

        pub fn get_screen(&self) -> usize {
            self.screen
        }

        pub fn exec<T: ScreenTrait + ?Sized>(&mut self, obj: &mut T) -> i32 {
            for _event in self.e.poll_iter() {
                match _event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => return -1,
                    Event::TextInput {
                        timestamp: _,
                        window_id: _,
                        text: _s,
                    } => {}
                    Event::KeyDown { keycode: key, .. } => {
                        self.screen = obj.keydown(self.screen, key.unwrap());
                    }
                    Event::KeyUp { keycode: key, .. } => {
                        self.screen = obj.keyup(self.screen, key.unwrap());
                    }
                    _ => {}
                }
            }
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.screen = obj.draw(self.screen, self);
            1
        }
    }
}
