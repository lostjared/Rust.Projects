
use libc::size_t;

#[link(name = "clib1", kind = "dylib")]
extern {
    fn hello_world() -> size_t;
}


fn main() {
    unsafe {
        hello_world();
    }
}
