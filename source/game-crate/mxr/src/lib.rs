pub mod mxr {

    use sdl2::rect::Rect;
    use sdl2::render::Canvas;
    use sdl2::render::TextureCreator;
    use sdl2::render::TextureQuery;
    use sdl2::video::WindowContext;
    use sdl2::EventPump;

    pub struct MXWindowBuilder {
        title: Option<String>,
        w: Option<u32>,
        h: Option<u32>,
    }

    impl Default for MXWindowBuilder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl MXWindowBuilder {
        pub fn new() -> Self {
            MXWindowBuilder {
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
        pub fn build(self) -> Result<MXWindow, String> {
            let sdl1 = sdl2::init().unwrap();
            let video1 = sdl1.video().unwrap();
            let wx = self.w.unwrap();
            let hx = self.h.unwrap();
            let titlex = self.title.unwrap();
            let window = video1.window(&titlex, wx, hx).opengl().build().unwrap();
            let can1 = window.into_canvas().build().map_err(|e| e.to_string())?;
            let tc1 = can1.texture_creator();
            let e = sdl1.event_pump().map_err(|x| x.to_string())?;

            Ok(MXWindow {
                title: titlex,
                w: wx,
                h: hx,
                sdl: sdl1,
                video: video1,
                can: can1,
                tc: tc1,
                event: e,
            })
        }
    }

    pub struct MXWindow {
        pub title: String,
        pub w: u32,
        pub h: u32,
        pub sdl: sdl2::Sdl,
        pub video: sdl2::VideoSubsystem,
        pub can: Canvas<sdl2::video::Window>,
        pub tc: TextureCreator<WindowContext>,
        pub event: EventPump,
    }

    impl MXWindow {
        pub fn printtext(
            &mut self,
            font: &sdl2::ttf::Font,
            x: i32,
            y: i32,
            color: sdl2::pixels::Color,
            text: &str,
        ) -> Result<(), String> {
            let text_surf = font
                .render(text)
                .blended(color)
                .map_err(|x| x.to_string())?;
            let text_surf_tex = self
                .tc
                .create_texture_from_surface(&text_surf)
                .map_err(|x| x.to_string())?;
            let tex_s = tex_get_size(&text_surf_tex);
            self.can.copy(
                &text_surf_tex,
                Some(Rect::new(0, 0, tex_s.0, tex_s.1)),
                Some(Rect::new(x, y, tex_s.0, tex_s.1)),
            )?;
            Ok(())
        }

        pub fn printtext_texture<'a>(
            &mut self,
            font: &sdl2::ttf::Font,
            tc: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
            color: sdl2::pixels::Color,
            text: &str,
        ) -> Result<sdl2::render::Texture<'a>, String> {
            let text_surf = font
                .render(text)
                .blended(color)
                .map_err(|x| x.to_string())?;
            let text_surf_tex = tc
                .create_texture_from_surface(&text_surf)
                .map_err(|x| x.to_string())?;
            Ok(text_surf_tex)
        }

        pub fn load_texture<'a>(
            &mut self,
            creator: &'a TextureCreator<WindowContext>,
            filename: &str,
            key: Option<sdl2::pixels::Color>,
        ) -> Result<sdl2::render::Texture<'a>, String> {
            let mut surf = sdl2::surface::Surface::load_bmp(filename)?;
            if key != None {
                surf.set_color_key(true, key.unwrap())?;
            }
            Ok(creator.create_texture_from_surface(&surf).unwrap())
        }

        pub fn load_gfx<'a>(
            &mut self,
            file_strings: Vec<&str>,
            tc: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
            key: Option<sdl2::pixels::Color>,
        ) -> Result<Vec<sdl2::render::Texture<'a>>, String> {
            let mut v = Vec::new();
            for i in file_strings {
                let t = self.load_texture(tc, i, key)?;
                v.push(t);
            }
            Ok(v)
        }
    }

    pub fn tex_get_size(tex: &sdl2::render::Texture) -> (u32, u32) {
        let TextureQuery {
            width: wi,
            height: hi,
            ..
        } = tex.query();
        (wi, hi)
    }
}
