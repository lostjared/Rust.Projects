use rand::Rng;

#[test]
fn check_if_sorted() {
    let v = gen_vec();
    for i in 0..v.len() - 1 {
        assert!(v[i + 1] >= v[i]);
    }
}

/// generate a vector of random numbers, sort it
fn gen_vec() -> Vec<u8> {
    let mut v = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        v.push(rng.gen_range(0..100));
    }
    v.sort();
    v
}

fn main() {
    println!("Hello, world!");
}
