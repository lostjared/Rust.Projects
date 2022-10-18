// system call for list of text arguments
use libc::system;
use libc::c_char;
use std::ffi::CString;

fn proc_list<T>(reader: T, cmd: &str) 
where
    T: std::io::BufRead + Sized, {
    if cmd.find("%f") == None {
        eprintln!("Error required %f");
        return;
    }
    for i in reader.lines() {
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
    if args.len() == 2 {
        proc_list(std::io::stdin().lock(), &args[1]);
    } else if args.len() == 3 {
        let f = std::fs::File::open(&args[1])?;
        let r = std::io::BufReader::new(f);
        proc_list(r, &args[2]);
    } else {
        eprintln!("Requires one or two arguments: input_list.txt \"command\"");
        return Ok(());
    }
    Ok(())
}
