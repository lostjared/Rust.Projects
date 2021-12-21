pub mod console_system {

    use sdl2::rect::Rect;
    use sdl2::render::TextureQuery;
    use std::process::Command;
    use std::process::Stdio;
    pub struct Console {
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        text: String,
        input_text: String,
        line_height: usize,
    }

    pub fn printtext(
        can: &mut sdl2::render::Canvas<sdl2::video::Window>,
        tex: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        font: &sdl2::ttf::Font,
        x: i32,
        y: i32,
        color: sdl2::pixels::Color,
        text: &str,
    ) {
        let text_surf = font.render(text).blended(color).unwrap();
        let text_surf_tex = tex.create_texture_from_surface(&text_surf).unwrap();
        let TextureQuery {
            width: wi,
            height: hi,
            ..
        } = text_surf_tex.query();
        can.copy(
            &text_surf_tex,
            Some(Rect::new(0, 0, wi, hi)),
            Some(Rect::new(x, y, wi, hi)),
        )
        .expect("on font copy");
    }

    pub fn printtext_width(
        blink: bool,
        line_height: &mut usize,
        can: &mut sdl2::render::Canvas<sdl2::video::Window>,
        tex: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        font: &sdl2::ttf::Font,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        color: sdl2::pixels::Color,
        text: &str,
    ) {
        if text.len() == 0 {
            return;
        }

        let mut vlst: Vec<String> = Vec::new();
        let mut width = x;
        let metrics = font.find_glyph_metrics('A').unwrap();
        let mut ypos = y;
        let mut value = String::new();

        for ch in text.chars() {
            if (width + metrics.advance > (w - 25) as i32) || ch == '\n' {
                vlst.push(value);
                value = String::new();
                if ch != '\n' {
                    value.push(ch);
                }
                ypos += metrics.advance + metrics.maxy;
                width = x;
            } else {
                value.push(ch);
                width += metrics.advance;
            }
        }
        if value.len() > 0 {
            vlst.push(value);
        }

        let mut yy = y;
        let mut line_index: usize = 0;

        for i in &vlst {
            if i.len() > 0 {
                printtext(can, tex, font, x, yy, color, i);
            }
            yy += metrics.advance + metrics.maxy;
            line_index += 1;
            if yy > h as i32 - 25 {
                *line_height = line_index;
                break;
            }
        }

        if blink == true {
            can.set_draw_color(color);
            can.fill_rect(Rect::new(
                width + 5,
                ypos,
                8,
                (metrics.maxy + metrics.advance) as u32,
            ))
            .expect("failed on rect");
        }
    }

    impl Console {
        pub fn new(xx: i32, yx: i32, wx: u32, hx: u32) -> Console {
            Console {
                x: xx,
                y: yx,
                w: wx,
                h: hx,
                text: String::new(),
                input_text: String::new(),
                line_height: 27,
            }
        }
        pub fn print(&mut self, t: &str) {
            self.text.push_str(t);
        }

        pub fn println(&mut self, t: &str) {
            self.text.push_str(t);
            self.text.push_str("\n");
        }

        pub fn type_key(&mut self, t: &str) {
            self.input_text.push_str(t);
            self.print(t);
        }

        pub fn back(&mut self) {
            if self.input_text.len() > 0 {
                self.input_text.pop();
                self.text.pop();
            }
        }

        pub fn proc_command(&mut self, v: Vec<&str>) {
            let name = v[0];
            match name {

                "exit" => {
                    std::process::exit(0);
                }

                "clear" => {
                    self.text.clear();
                }

                "exec" => {
                    if v.len() >= 2 {
                        let name = v[1];
                        let output;
                        if v.len() > 2 {
                            let args = &v[2..v.len()];
                            output = Command::new(name)
                                .args(args)
                                .stdout(Stdio::piped())
                                .output();
                        } else if v.len() == 2 {
                            output = Command::new(name).stdout(Stdio::piped()).output();
                        } else {
                            self.println("Error requires argument...\n");
                            self.print("cmd=)>");
                            return;
                        }

                        match output {
                            Ok(output) => {
                                let stdout = String::from_utf8(output.stdout).unwrap();
                                self.print("\n");
                                self.print(&stdout);
                            }
                            _ => {
                                self.print("\n");
                                let s = format!("{:?}", output.unwrap_err());
                                self.println(&s);
                            }
                        }
                    } else {
                        self.println("\nError requires argument of command...");
                    }
                }
                _ => { self.print("\n"); }
            }
            self.input_text = String::new();
            self.print("cmd=)>");
        }

        pub fn enter(&mut self) {
            // proc command
            let input = String::from(&self.input_text);
            let v: Vec<&str> = input.split(' ').collect();
            if v.len() == 0 {
                self.print("\n");
                self.print("cmd=)>");
                return;
            }
            self.proc_command(v);
        }

        pub fn draw(
            &mut self,
            blink: bool,
            can: &mut sdl2::render::Canvas<sdl2::video::Window>,
            tex: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
            font: &sdl2::ttf::Font,
            color: sdl2::pixels::Color,
        ) {
            let f = self.text.find("\n");
            let l: Vec<_> = self.text.lines().collect();
            if f != None && l.len() > (self.line_height as usize) - 1 {
                let v = &self.text[f.unwrap() + 1..self.text.len()];
                self.text = String::from(v);
            }

            printtext_width(
                blink,
                &mut self.line_height,
                can,
                tex,
                font,
                self.x,
                self.y,
                self.w,
                self.h,
                color,
                &self.text,
            );
        }
    }
}
