
fn main() -> std::io::Result<()> {
    let args : Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        let val = &args[1];
        let sep = val.find(',');
        if sep == None {
            panic!("Missing required , ");
        }
        let sep = sep.unwrap();
        let num1 = &val[0..sep];
        let num2 = &val[sep+1..];

        let start_pos: usize = num1.parse().unwrap();
        let stop_pos: usize = num2.parse().unwrap();

        

    }
    Ok(())
}