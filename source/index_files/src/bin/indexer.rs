
use index_files::indexer::index_copy;
use opt::argz;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let mut input_file = String::new();
    let mut output_file= String::new();
    let mut name_file = String::new();

    let _rt_val = argz::getopt(&args, "i:o:n:", |i: char, param: String| match i {
        'i' => {
            input_file = param.to_owned();
        }
        'o' => {
            output_file = param.to_owned();
        }
        'n' => {
            name_file = param.to_owned();
        }
        _ => {
        }
    });
    if input_file.is_empty() || output_file.is_empty() {
        println!("use:\n\t -i input file\n\t -o output directory\n\t -n optional prefix name\n");
        println!("{}:{}:{}", input_file, output_file, name_file);
        return;
    }
    index_copy(&input_file, &output_file, &name_file);
}