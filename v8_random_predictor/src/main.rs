use std::io::{stdin};

#[derive(Debug)]
struct Xorshift128 {
    state1: u64,
    state2: u64,
}

impl Xorshift128 {
    fn new(state1: u64, state2: u64) -> Self {
        Xorshift128 { state1, state2 }
    }

    fn next(&mut self) -> Self {
        let mut s1: u64 = self.state1;
        let s2: u64 = self.state2;

        s1 ^= s1 << 23;
        s1 ^= s1 >> 17;
        s1 ^= s2;
        s1 ^= s2 >> 26;

        Xorshift128::new(s2, s1)
    }
}

fn main() {
    let mut sample: Xorshift128 = Xorshift128::new(12345,67890);

    for _ in 0..10 {
        let temp: Xorshift128 = Xorshift128::next(&mut sample);
        sample = temp;

        println!("{sample:#?}")
    }
}
