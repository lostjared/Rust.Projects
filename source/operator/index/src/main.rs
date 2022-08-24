
use std::ops::{Index, IndexMut};


struct Values {
    v: Vec<usize>
}

impl Values {
    pub fn new() -> Values {
        Values {
            v: Vec::from(vec![0, 1, 2, 3])
        }
    }
}

impl Index<usize> for Values {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}

impl IndexMut<usize> for Values {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index]
    }
}


fn main() -> std::io::Result<()> {
    let mut v = Values::new();
    println!("{}:{}", v[0], v[1]);
    v[0] = 1;
    println!("{}:{}", v[0], v[1]);
    Ok(())
}
