// view map
// use arrow keys to scroll
// pass arguments
// cargo run -- levelfile.lvl levelgraphics.gfx

use clap::{App, Arg};
use rs_catgfx::catgfx::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};
use tilemap::tile_map::*;
/// draw map
fn draw_map(
    can: &mut sdl2::render::WindowCanvas,
    cam: &Camera,
    tmap: &TileMap,
    textures: &Vec<sdl2::render::Texture>,
) {
    let tsize = 16;
    let start_col = cam.x / tsize;
    let end_col = start_col + (cam.width / tsize);
    let start_row = cam.y / tsize;
    let end_row = start_row + (cam.height / tsize);
    let cx = cam.x;
    let cy = cam.y;
    let off_x = -cx + start_col * tsize;
    let off_y = -cy + start_row * tsize;
    for x in start_col..end_col {
        for y in start_row..end_row {
            if x >= 0 && x < tmap.width && y >= 0 && y < tmap.height {
                let tile = tmap.at(x, y);
                if tile != None {
                    let tile = tile.unwrap();
                    let xx: i32 = (x - start_col) * tsize + off_x;
                    let yy: i32 = (y - start_row) * tsize + off_y;
                    if tile.solid != 2 && tile.solid != 3 {
                        can.copy(
                            &textures[tile.img as usize],
                            sdl2::rect::Rect::new(0, 0, 16, 16),
                            sdl2::rect::Rect::new(xx, yy, 16, 16),
                        )
                        .expect("on copy");
                    }
                }
            }
        }
    }
}

fn draw_map_objects(
    can: &mut sdl2::render::WindowCanvas,
    cam: &Camera,
    tmap: &TileMap,
    obj_tex: &Vec<sdl2::render::Texture>,
) {
    let tsize = 16;
    let start_col = cam.x / tsize;
    let end_col = start_col + (cam.width / tsize);
    let start_row = cam.y / tsize;
    let end_row = start_row + (cam.height / tsize);
    let cx = cam.x;
    let cy = cam.y;
    let off_x = -cx + start_col * tsize;
    let off_y = -cy + start_row * tsize;
    for x in start_col - 16..end_col {
        for y in start_row - 16..end_row {
            if x >= 0 && x < tmap.width && y >= 0 && y < tmap.height {
                let tile = tmap.at(x, y);
                if tile != None {
                    let tile = tile.unwrap();
                    if tile.layers[0] > 0 && tile.layers[0] <= 7 {
                        let xx: i32 = (x - start_col) * tsize + off_x;
                        let yy: i32 = (y - start_row) * tsize + off_y;
                        if tile.solid != 2 && tile.solid != 3 {
                            let layer = tile.layers[0] - 1;
                            let sdl2::render::TextureQuery {
                                width: wi,
                                height: hi,
                                ..
                            } = obj_tex[layer as usize].query();

                            can.copy(
                                &obj_tex[layer as usize],
                                sdl2::rect::Rect::new(0, 0, wi, hi),
                                sdl2::rect::Rect::new(xx, yy, wi, hi),
                            )
                            .expect("on copy");
                        }
                    }
                }
            }
        }
    }
}

/// build table of surfaces
fn build_map(filename: &str) -> Vec<(sdl2::surface::Surface, (u32, u32, u32))> {
    let mut table: GfxTable = GfxTable::new();
    build_gfx(filename, &mut table).expect("building graphics table");
    let mut surf: Vec<(sdl2::surface::Surface, (u32, u32, u32))> = Vec::new();
    // load graphics
    for i in &table.items {
        let mut rwops = sdl2::rwops::RWops::from_bytes(i.data.as_slice()).unwrap();
        let s = sdl2::surface::Surface::load_bmp_rw(&mut rwops).unwrap();
        surf.push((s, (i.index, i.solid, i.obj)));
    }
    surf
}

fn fmin(x: f64, x2: f64) -> f64 {
    if x < x2 {
        x
    } else {
        x2
    }
}

struct Arguments {
    map: String,
    gfx: String,
}

fn parse_args() -> Arguments {
    let m = App::new("tiledmeo")
        .arg(
            Arg::new("map")
                .takes_value(true)
                .required(true)
                .long("map")
                .short('m')
                .allow_invalid_utf8(true)
                .help("map file"),
        )
        .arg(
            Arg::new("gfx")
                .takes_value(true)
                .required(true)
                .long("gfx")
                .short('g')
                .allow_invalid_utf8(true)
                .help("graphics file"),
        )
        .get_matches();
    let map_ = m.value_of_lossy("map").unwrap();
    let gfx_ = m.value_of_lossy("gfx").unwrap();

    Arguments {
        map: map_.to_string(),
        gfx: gfx_.to_string(),
    }
}

/// main function
fn main() -> std::io::Result<()> {
    let args = parse_args();

    let mut tmap: TileMap = TileMap::new();

    if args.map.find(".lvl") != None {
        tmap.load_map(&args.map)?;
    } else if args.map.find(".txt") != None {
        tmap.load_map_text(&args.map)?;
    } else {
        println!("filename must end in .lvl or .txt");
        std::process::exit(0);
    }

    println!(
        "Map loaded: [{}] - {}x{}",
        tmap.name, tmap.width, tmap.height
    );
    let surfaces: Vec<(sdl2::surface::Surface, (u32, u32, u32))> = build_map(&args.gfx);
    println!("Images loaded: {}", surfaces.len());
    let max_x = tmap.width * 16 - 1280 - 1;
    let max_y = tmap.height * 16 - 720 - 1;
    let mut cam: Camera = Camera::new(1280, 720, max_x, max_y);
    let width = 1280 - 32;
    let height = 720 - 32;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("Tile Demo", width, height)
        .resizable()
        .opengl()
        .build()
        .unwrap();
    let mut can = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .expect("Error on canvas");
    let tc = can.texture_creator();
    let mut textures: Vec<sdl2::render::Texture> = Vec::new();
    let mut obj_text: Vec<sdl2::render::Texture> = Vec::new();
    for mut i in surfaces {
        i.0.set_color_key(true, sdl2::pixels::Color::RGBA(255, 255, 255, 255))
            .expect("on set color key");
        let tex = tc.create_texture_from_surface(i.0).unwrap();
        match i.1 .2 {
            0 => {
                textures.push(tex);
            }
            1 => {
                obj_text.push(tex);
            }
            _ => {}
        }
    }

    let mut e = sdl.event_pump().unwrap();
    let mut prev_tick: u64 = 0;
    let mut amt = 0;
    'main: loop {
        can.clear();
        let start = SystemTime::now();
        let se = start.duration_since(UNIX_EPOCH).expect("error on time");
        let tick = se.as_secs() * 1000 + se.subsec_nanos() as u64 / 1_000_000;
        let mut delta: f64 = (tick as f64 - prev_tick as f64) / 1000.0;
        let timeout = tick - prev_tick;
        prev_tick = tick;
        delta = fmin(0.95, delta);
        amt += timeout;

        for _event in e.poll_iter() {
            match _event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }
        let keys: HashSet<_> = e
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut move_x: i32 = 0;
        let mut move_y: i32 = 0;

        for i in &keys {
            match *i {
                Keycode::Left => {
                    move_x = -1;
                }
                Keycode::Right => {
                    move_x = 1;
                }
                Keycode::Up => {
                    move_y = -1;
                }
                Keycode::Down => {
                    move_y = 1;
                }
                _ => {}
            }
        }
        if amt > 10 {
            amt = 0;
            if move_x != 0 || move_y != 0 {
                cam.move_camera(0.0125, move_x, move_y);
            }
        }
        draw_map(&mut can, &cam, &tmap, &textures);
        draw_map_objects(&mut can, &cam, &tmap, &obj_text);
        can.present();
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
