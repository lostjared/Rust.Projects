fn main() {
    let c_str = std::ffi::CString::new("Hello, World!\n").unwrap();
    let w: *const libc::c_char = c_str.as_ptr() as *const libc::c_char;
    unsafe {
        libc::puts(w);
    }
}
