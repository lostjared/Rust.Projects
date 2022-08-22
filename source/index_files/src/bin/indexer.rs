
use index_files::indexer::index_copy;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() <= 2 || args.len() > 3 {
        println!("use:\n\t{}: input_list.txt output_dir", args.get(0).unwrap());
        return;
    }
    index_copy(args.get(1).unwrap(), args.get(2).unwrap());
}