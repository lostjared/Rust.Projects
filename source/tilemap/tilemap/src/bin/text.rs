use tilemap::tile_map::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Requires one argument");
    }

    let mut tmap = TileMap::new();
    tmap.load_map_text(&args[1])?;

    Ok(())
}
