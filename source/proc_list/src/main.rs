// system call for list of text arguments
use std::io::BufRead;
use libc::system;
use libc::c_char;
use std::ffi::CString;

fn proc_list(input: &str, cmd: &str) {
    if cmd.find("%f") == None {
        eprintln!("Error required %f");
        return;
    }
    let f = std::fs::File::open(input).expect("File open");
    let r = std::io::BufReader::new(f);
    for i in r.lines() {
        match i {
            Ok(value) => {
                let command = cmd.replace("%f", &value);
                println!("{}", command);
                unsafe {
                    let cmd = CString::new(command).unwrap();
                    let c_string: *const c_char = cmd.as_ptr() as *const c_char;
                    system(c_string);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let args : Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Requires two arguments: input_list.txt \"command\"");
        return Ok(());
    }

    proc_list(&args[1], &args[2]);
    Ok(())
}
