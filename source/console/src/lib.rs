pub mod console_system {

    use sdl2::rect::Rect;
    use sdl2::render::TextureQuery;
    use std::process::Command;
    use std::process::Stdio;
    use logger::log::*;

    /// Console struct containing information for console
    pub struct Console {
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        text: String,
        input_text: String,
        line_height: usize,
        color: sdl2::pixels::Color,
        visible: bool,
        log: Log,
    }

    /// printtext function for printing text to the screen
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

    /// printtext width function for printing text to the screen aligned by a certain width
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
        if text.is_empty() {
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
        if !value.is_empty() {
            vlst.push(value);
        }

        let mut yy = y;
        let mut line_index: usize = 0;

        for i in &vlst {
            if !i.is_empty() {
                printtext(can, tex, font, x, yy, color, i);
            }
            yy += metrics.advance + metrics.maxy;
            line_index += 1;
            if yy > h as i32 - 25 {
                *line_height = line_index;
                break;
            }
        }

        if blink {
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
        /// create a new console
        pub fn new(xx: i32, yx: i32, wx: u32, hx: u32) -> Console {
            let home_dir = dirs::home_dir();
            match home_dir {
                Some(hdir) => {
                    std::env::set_current_dir(hdir).expect("could not set directory");
                }
                None => {
                    println!("no home directory");
                }
            }

            let mut log_ = Log::new_file_log("console", "log.txt", true, true);
            log_.i("Console started up");

            Console {
                x: xx,
                y: yx,
                w: wx,
                h: hx,
                text: String::new(),
                input_text: String::new(),
                line_height: 27,
                color: sdl2::pixels::Color::RGB(255, 255, 255),
                visible: true,
                log: log_
            }
        }

        /// set console text color
        pub fn set_text_color(&mut self, col: sdl2::pixels::Color) {
            self.color = col;
        }

        /// set console is visible or not
        pub fn set_visible(&mut self, v: bool) {
            self.visible = v;
        }

        /// get console visible or not
        pub fn get_visible(&mut self) -> bool {
            self.visible
        }

        /// change console directory
        pub fn change_dir(&mut self, d: &str) {
            let result = std::env::set_current_dir(std::path::Path::new(d));
            match result {
                Ok(_) => {}
                Err(s) => {
                    self.println(&format!("\nError could not change directory... {}", s));
                }
            }
        }

        /// print text to the console
        pub fn print(&mut self, t: &str) {
            self.text.push_str(t);
        }

        /// print text to the console with trailing newline
        pub fn println(&mut self, t: &str) {
            self.text.push_str(t);
            self.text.push('\n');
        }

        /// input keypress to the console
        pub fn type_key(&mut self, t: &str) {
            if self.visible {
                self.input_text.push_str(t);
                self.print(t);
            }
        }

        /// input backspace key to the console
        pub fn back(&mut self) {
            if self.visible && !self.input_text.is_empty() {
                self.input_text.pop();
                self.text.pop();
            }
        }

        /// print the prompt
        pub fn print_prompt(&mut self) {
            let path = std::env::current_dir().unwrap();
            self.print(&format!("[{}]=)>", path.display()));
        }

        /// process a shell command
        pub fn proc_command(&mut self, v: Vec<&str>, cmd: &str) {
            let name = v[0];
            match name {
                "cd" => {
                    if v.len() != 2 {
                        self.println("\n Requires path...\n");
                    } else {
                        self.change_dir(v[1]);
                        self.print("\n");
                    }
                }
                "setcolor" => {
                    if v.len() != 4 {
                        self.println("\nError requires r g b arguments...\n");
                    } else {
                        let r = v[1].parse::<u8>();
                        let g = v[2].parse::<u8>();
                        let b = v[3].parse::<u8>();

                        let rr: u8;
                        let gg: u8;
                        let bb: u8;

                        match r {
                            Ok(r_r) => {
                                rr = r_r;
                            }
                            Err(_) => {
                                self.println("\nError on setcolor");
                                self.input_text = String::new();
                                self.print_prompt();
                                return;
                            }
                        }
                        match g {
                            Ok(g_g) => {
                                gg = g_g;
                            }
                            Err(_) => {
                                self.println("\nError on setcolor");
                                self.input_text = String::new();
                                self.print_prompt();
                                return;
                            }
                        }

                        match b {
                            Ok(b_b) => {
                                bb = b_b;
                            }
                            Err(_) => {
                                self.println("\nError on setcolor");
                                self.input_text = String::new();
                                self.print_prompt();
                                return;
                            }
                        }

                        self.color = sdl2::pixels::Color::RGB(rr, gg, bb);
                        self.println("\nColor set.\n");
                    }
                }
                "shell" => {
                    let output;
                    if !v.is_empty() && cmd.len() > 6 {
                        let icmd = &cmd[6..cmd.len()];
                        output = Command::new("/bin/sh")
                            .arg("-c")
                            .arg(icmd)
                            .stdout(Stdio::piped())
                            .output();

                        match output {
                            Ok(output) => {
                                let stdout = String::from_utf8(output.stdout).unwrap();
                                self.print("\n");
                                self.print(&stdout);
                            }
                            Err(s) => {
                                self.print("\n");
                                let s = format!("{}", s);
                                self.println(&s);
                            }
                        }
                    } else {
                        self.println("\nError invalid command..");
                    }
                }

                "about" => {
                    self.println("\nRust SDL Console v1.0. https://github.com/lostjared\nhttp://lostsidedead.com");
                }

                "exit" => {
                    std::process::exit(0);
                }

                "clear" => {
                    self.text.clear();
                }
                "hide" => {
                    self.set_visible(false);
                    self.print("\n");
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
                            self.print_prompt();
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
                _ => {
                    self.print("\n");
                }
            }
            self.input_text = String::new();
            self.print_prompt();
        }

        /// send enter key to console
        pub fn enter(&mut self) {
            if !self.visible {
                return;
            }

            // proc command
            let input = String::from(&self.input_text);
            let v: Vec<&str> = input.split(' ').collect();
            if v.is_empty() {
                self.print("\n");
                self.print_prompt();
                return;
            }
            self.proc_command(v, &input);
            self.log.i(&format!("command: {}", input));
        }

        /// draw the console
        pub fn draw(
            &mut self,
            blink: bool,
            can: &mut sdl2::render::Canvas<sdl2::video::Window>,
            tex: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
            font: &sdl2::ttf::Font,
        ) {
            if !self.visible {
                return;
            }

            let f = self.text.find('\n');
            if f != None && self.text.lines().count() > (self.line_height as usize) - 1 {
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
                self.color,
                &self.text,
            );
        }
    }
}
