
use std::process::Command;

fn main() {
    let mut cmd = Command::new("ls");
    cmd.arg("-l");
    cmd.status().expect("list directory");
}