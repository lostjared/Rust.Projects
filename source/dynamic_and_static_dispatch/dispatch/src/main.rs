

trait Draw {
    fn draw_to_screen(&self);
}

struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

impl Draw for Vertex {
    fn draw_to_screen(&self) {
        println!("draw {}x{}x{}", self.x, self.y, self.z);
    }
}

fn dynamic_dispatch(d: &dyn Draw) {
    d.draw_to_screen();
}

fn static_dispatch<T: Draw>(d: &T) {
    d.draw_to_screen();
}

fn main() {


    let v = Vertex { x: 0.0, y: 0.0, z: 0.0 };
    dynamic_dispatch(&v);
    static_dispatch(&v);

}