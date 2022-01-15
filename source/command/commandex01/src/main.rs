use std::process::Command;

fn main() {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg("echo Hello_World");
    cmd.spawn().expect("Error");
}
