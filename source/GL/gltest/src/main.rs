extern crate gl;
extern crate sdl2;

use std::ffi::CStr;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("OpenGL app", 1280, 720)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 1280, 720);
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        let version = gl::GetString(gl::VERSION) as *const i8;
        let v = String::from_utf8(CStr::from_ptr(version).to_bytes().to_vec()).unwrap();
        println!("OpenGL Version: {}", v);
        let vendor = gl::GetString(gl::VENDOR) as *const i8;
        let ven = String::from_utf8(CStr::from_ptr(vendor).to_bytes().to_vec()).unwrap();
        println!("OpenGL Vendor: {}", ven);
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }
}
