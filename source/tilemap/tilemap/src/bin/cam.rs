use tilemap::tile_map::*;

fn main() -> std::io::Result<()> {
    let max_x = (1280 / 16 * 2) * 16 - 1280 - 1;
    let max_y = (720 / 16 * 2) * 16 - 720 - 1;
    let mut c: Camera = Camera::new(1280, 720, max_x, max_y);
    c.move_camera(1.0, 1, 1);
    println!("Camera pos: {}, {}", c.x, c.y);
    c.reset();
    println!("Camera pos: {}, {}", c.x, c.y);

    Ok(())
}
