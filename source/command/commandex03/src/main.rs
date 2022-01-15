use std::process::Command;

fn main() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo \"Hello World\""])
            .spawn()
            .expect("execfile");
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo \"Hello, World\"")
            .spawn()
            .expect("exec file");
    }
}
