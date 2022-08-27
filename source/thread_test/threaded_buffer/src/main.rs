
use::filter::filter::{FilterImage};
use rand::Rng;
use std::thread;
use std::sync::{Mutex, Arc};

fn process_chunk(buf: &mut [u8], y: usize, width: usize, chunk: usize, bpp: usize) {
    let mut rng = rand::thread_rng();
    let pitch = width*bpp;
    for z in y..y+chunk {
        for i in 0..width {
            let pos = z * pitch + (i * bpp);
            buf[pos] = rng.gen_range(0..255);
            buf[pos+1] = rng.gen_range(0..255);
            buf[pos+2] = rng.gen_range(0..255);
            buf[pos+3] = 255;
        }
    }
}

fn main() {

    let arguments : Vec<String> = std::env::args().collect();
    let filename = arguments.get(1).unwrap();
    let mut im = FilterImage::load_from_png(filename);
    let mut pos = 0;
    let num_threads = 8;
    let len = im.bytes.len();
    let chunk = im.height/num_threads;
    let width = im.width;
    let bpp = im.bpp;
    let mut handles = vec![];
    let bytes = im.bytes;
    let values = Arc::new(Mutex::new(bytes));
    for _i in 0..num_threads {
        let p = pos.clone();
        let v = values.clone();
        handles.push(thread::spawn(move || {
            let mut val = v.lock().unwrap();
            process_chunk(&mut val[..len], p, width, chunk, bpp);
        }));
        pos += chunk;
    }
    for j in handles {
        j.join().unwrap();
    }
    im.bytes = values.lock().unwrap().clone();
    im.save_to_file("output.png");
    println!("wrote to file: output.png");
}
