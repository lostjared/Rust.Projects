use tilemap::tile_map::*;

fn main() -> std::io::Result<()> {
    // collect arguments
    let args: Vec<String> = std::env::args().collect();
    // create new tile map struct

    let mut m = TileMap::new();
    // if args not equal 2 quit
    if args.len() != 2 {
        eprintln!("Error invalid arguments....\n");
        std::process::exit(-1);
    }
    // load map from file
    m.load_map(&args[1])?;

    Ok(())
}
