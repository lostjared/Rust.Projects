extern crate rmx;
use rmx::rmx_system;

struct Screen1 {}
impl Screen1 {
    pub fn load(&mut self) {}
}

struct Screen2 {}
impl Screen2 {
    pub fn load(&mut self) {}
}

impl rmx_system::ScreenTrait for Screen2 {
    fn draw(&mut self, scr: usize, system: &mut rmx_system::System) -> usize {
        system.can.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        system.can.fill_rect(None).expect("Fill rect");
        scr
    }
    fn keydown(&mut self, scr: usize, key: sdl2::keyboard::Keycode) -> usize {
        println!("key down: {}", key);
        if key == sdl2::keyboard::Keycode::Return {
            0
        } else {
            scr
        }
    }
    fn keyup(&mut self, scr: usize, key: sdl2::keyboard::Keycode) -> usize {
        println!("key up: {}", key);
        scr
    }
}

impl rmx_system::ScreenTrait for Screen1 {
    fn draw(&mut self, scr: usize, system: &mut rmx_system::System) -> usize {
        system
            .can
            .set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        system.can.fill_rect(None).expect("Fill rect");
        scr
    }
    fn keydown(&mut self, scr: usize, key: sdl2::keyboard::Keycode) -> usize {
        println!("key down: {}", key);
        if key == sdl2::keyboard::Keycode::Space {
            1
        } else {
            scr
        }
    }
    fn keyup(&mut self, scr: usize, key: sdl2::keyboard::Keycode) -> usize {
        println!("key up: {}", key);
        scr
    }
}

fn main() {
    let mut sys = rmx_system::System::init("Skeleton", 1280, 720);
    let mut scr = Screen1 {};
    let mut scr2 = Screen2 {};
    scr.load();
    scr2.load();
    sys.set_screen(0);
    'main: loop {
        match sys.get_screen() {
            0 => {
                if sys.exec(&mut scr) == -1 {
                    break 'main;
                }
            }
            1 => {
                if sys.exec(&mut scr2) == -1 {
                    break 'main;
                }
            }
            _ => {}
        }
    }
}
