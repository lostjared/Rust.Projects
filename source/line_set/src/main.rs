// enter line number plus string
// then type list to have them listed in the order of the line number

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {

    }
    let mut line_map : HashMap<usize, String> = HashMap::new();
    let mut line_set : HashSet<usize> = HashSet::new();


    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let iline = line;
                let pos = iline.find(' ');
                if pos != None {
                    let p = pos.unwrap();
                    let left = &iline[0..p];
                    let index = left.trim().parse::<i32>();
                    match index {
                        Ok(num) => {
                            line_map.insert(num as usize, iline);
                            line_set.insert(num as usize);
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                } else if iline.eq("list") {
                    let mut v : Vec<usize> = Vec::new();
                    for i in &line_set {
                        v.push(*i);
                    }
                    v.sort();
                     for i in &v {
                        let index = line_map.get(i).unwrap();
                        println!("{}", index);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
               // println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
             //   println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

}
