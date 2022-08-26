

fn fill_slice(out: &mut [u8]) {
    for i in 0..out.len() {
        out[i] = (i*2) as u8;
    }
}

fn echo_slice(out: &[u8]) {
    let mut index = 1;
    for i in out {
        println!("{}:\t{}", index, i);
        index += 1;
    }
}


fn main() -> std::io::Result<()> {
    let mut v = vec![1,2,3,4];
    let len = v.len();
    fill_slice(&mut v[0..len]);
    echo_slice(&v[0..len]);
    let o = &mut v[0..len];
    echo_slice(o);
    Ok(())
}