async fn hello_world() {
    println!("Hello, World!");
}

fn main() -> std::io::Result<()> {
    futures::executor::block_on(hello_world());
    Ok(())
}
