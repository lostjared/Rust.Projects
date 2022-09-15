


async fn one() {
    for i in 0..10 {
        println!("Hello World: {}", i);
    }
}

async fn two() {
    for i in 0..10 {
        println!("Hey: {}", i);
    }
}

async fn three() {
    let one_val = one();
    let two_val = two();
    futures::join!(one_val, two_val);
}

async fn four() {
    one().await;
    two().await;
}

fn main() -> std::io::Result<()> {
    futures::executor::block_on(three());
    futures::executor::block_on(four());
    Ok(())
}