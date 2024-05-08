pub mod mxr {

    use sdl2::render::Canvas;
    use sdl2::render::TextureCreator;
    use sdl2::video::WindowContext;
    use sdl2::EventPump;

    pub struct WindowBuilder {
        title: Option<String>,
        w: Option<u32>,
        h: Option<u32>,
    }

    impl Default for WindowBuilder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl WindowBuilder {
        pub fn new() -> Self {
            WindowBuilder {
                title: None,
                w: None,
                h: None,
            }
        }
        pub fn create(mut self, cap: &str, w: u32, h: u32) -> Self {
            self.title = Some(String::from(cap));
            self.w = Some(w);
            self.h = Some(h);
            self
        }
        pub fn build(self) -> Window {
            let sdl1 = sdl2::init().unwrap();
            let video1 = sdl1.video().unwrap();
            let window = video1
                .window(
                    &self.title.clone().expect("title"),
                    self.w.expect("width"),
                    self.h.expect("on h"),
                )
                .opengl()
                .build()
                .unwrap();
            let can1 = window
                .into_canvas()
                .build()
                .map_err(|e| e.to_string())
                .expect("Error on canvas");
            let tc1 = can1.texture_creator();
            let e = sdl1.event_pump().unwrap();

            Window {
                title: self.title.expect("title"),
                w: self.w.expect("width"),
                h: self.h.expect("height"),
                sdl: sdl1,
                video: video1,
                can: can1,
                tc: tc1,
                event: e,
            }
        }
    }

    pub struct Window {
        pub title: String,
        pub w: u32,
        pub h: u32,
        pub sdl: sdl2::Sdl,
        pub video: sdl2::VideoSubsystem,
        pub can: Canvas<sdl2::video::Window>,
        pub tc: TextureCreator<WindowContext>,
        pub event: EventPump,
    }
}
