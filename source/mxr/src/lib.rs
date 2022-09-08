
pub mod mxr {

    use sdl2::pixels::Color;

    pub trait Screen {
        fn draw(&mut self, can: &mut sdl2::render::Canvas<sdl2::video::Window>, width: u32, height: u32);
        fn event(&mut self, e: &sdl2::event::Event);
    }

    pub struct ScreenObjects {

        scr: Vec<Box<dyn Screen>>,
        cur_screen: usize,
        width: u32,
        height: u32,
    }

    impl ScreenObjects {
        pub fn new(w: u32, h: u32) -> Self {
            Self {
                scr: Vec::new(),
                cur_screen: 0,
                width: w,
                height: h,
            }
        }
        pub fn push_screen(&mut self, screen: Box<dyn Screen>) {
            self.scr.push(screen);
        }

        pub fn draw(&mut self, can: &mut sdl2::render::Canvas<sdl2::video::Window>) {
            self.scr[self.cur_screen].draw(can, self.width, self.height);
        }

        pub fn event(&mut self, e: &sdl2::event::Event) {
            self.scr[self.cur_screen].event(e);
        }

        pub fn set_screen(&mut self, screen: usize) {
            self.cur_screen = screen;
        }
    }

}
