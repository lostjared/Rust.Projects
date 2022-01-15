extern crate rmx;
use rmx::rmx_system;

static SCREEN1: usize = 0;
static SCREEN2: usize = 1;

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
        system
            .canvas
            .set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        system.canvas.fill_rect(None).expect("Fill rect");
        scr
    }
    fn keydown(&mut self, scr: usize, key: sdl2::keyboard::Keycode) -> usize {
        println!("key down: {}", key);
        if key == sdl2::keyboard::Keycode::Return {
            SCREEN1
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
            .canvas
            .set_draw_color(sdl2::pixels::Color::RGB(100, 0, 0));
        system.canvas.fill_rect(None).expect("Fill rect");
        scr
    }
    fn keydown(&mut self, scr: usize, key: sdl2::keyboard::Keycode) -> usize {
        println!("key down: {}", key);
        if key == sdl2::keyboard::Keycode::Space {
            SCREEN2
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
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let fontx = ttf_context
        .load_font("./font.ttf", 18)
        .expect("font load test");
    let mut system = rmx_system::System::init("Skeleton", 1280, 720);
    let tc = system.canvas.texture_creator();
    let _text_surf = fontx
        .render("init")
        .blended(sdl2::pixels::Color::RGB(255, 255, 255))
        .unwrap();
    system
        .console
        .set_text_color(sdl2::pixels::Color::RGB(255, 255, 255));
    system.console.set_visible(true);
    let mut scr = Screen1 {};
    let mut scr2 = Screen2 {};
    scr.load();
    scr2.load();
    system.set_screen(SCREEN1);
    let mut screens: Vec<Box<dyn rmx_system::ScreenTrait>> = Vec::new();
    screens.push(Box::new(scr));
    screens.push(Box::new(scr2));
    system.console.println("Hello, World!");
    let mut counter = 0;
    'main: loop {
        let value = &mut screens[system.get_screen()];
        let cur_screen = value.as_mut();

        match system.exec(cur_screen) {
            -1 => {
                break 'main;
            }
            _ => {
                system.console.println(&format!("Tick: {}", counter));
                counter += 1;
                system.console.draw(false, &mut system.canvas, &tc, &fontx);
                system.canvas.present();
            }
        }
    }
}
