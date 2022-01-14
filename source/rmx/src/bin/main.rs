extern crate rmx;

use rmx::rmx_system;

struct Screen1 {}

impl Screen1 {
    pub fn load(&mut self) {

    }
}

impl rmx_system::ScreenTrait for Screen1 {
    fn draw(&mut self, can: &mut sdl2::render::Canvas<sdl2::video::Window>) {

    }
    fn keydown(&mut self,key: sdl2::keyboard::Keycode) {
        println!("key down: {}", key);

    }
    fn keyup(&mut self, key: sdl2::keyboard::Keycode) {
        println!("key up: {}", key);
    }
}



fn main() {
    let mut sys = rmx_system::System::init("Skeleton", 1280, 720);
    let mut scr = Screen1 {};
    scr.load();
    sys.set_screen(0);
    match sys.get_screen() {
        0 => {
            sys.exec(&mut scr);
        }
        _ => {}
    }
}
