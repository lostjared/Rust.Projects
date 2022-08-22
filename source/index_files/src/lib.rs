
pub mod indexer {
    pub use std::io::BufRead;
    pub fn index_copy(filename: &str, output_dir: &str, new_name: &str) {
        let f = std::fs::File::open(filename).expect("on opening of file");
        let mut reader = std::io::BufReader::new(f);
        let mut files : Vec<String> = Vec::new();
        loop {
            let mut s : String = String::new();
            let len = reader.read_line(&mut s).expect("on readline");    
            if len == 0 {
                break;
            }
            files.push(String::from(s.trim()));
        }
        let mut num = 0;
        let sval = format!("{}", files.len()+1);
        let count = sval.len();
        for i in &files {
            let in_file = std::path::Path::new(i);
            let fname = in_file.file_stem().unwrap();
            let ext = in_file.extension().unwrap();
            let final_name;
            if new_name.is_empty() {
                final_name = format!("{}/{}.{:0width$}.{}", output_dir, fname.to_string_lossy(), num, ext.to_string_lossy(),width=count);
            } else {
                final_name = format!("{}/{}.{:0width$}.{}", output_dir, new_name, num, ext.to_string_lossy(),width=count);
            }
            let final_path = std::path::Path::new(&final_name);
            let final_s = final_path.to_string_lossy();
            std::fs::copy(i.to_string(), final_s.to_string()).expect("failed to rename");
            println!("{} -> {}", i, final_s);
            num += 1;
        }
   }
}