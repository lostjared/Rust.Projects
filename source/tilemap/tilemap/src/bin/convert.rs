use tilemap::tile_map::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Requires two arguments\ninput.lvl output.txt");
        std::process::exit(0);
    }
    convert(&args[1], &args[2])?;
    Ok(())
}

fn convert(input: &str, output: &str) -> std::io::Result<()> {
    let mut tmap = TileMap::new();
    tmap.load_map(input)?;
    tmap.save_map_text(output)?;
    println!("convert: converted {} to {}", input, output);
    Ok(())
}
