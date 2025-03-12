#[derive(Debug)]
struct Xorshift128 {
    state1: u64,
    state2: u64,
}

impl Xorshift128 {
    fn new(state1: u64, state2: u64) -> Self {
        Xorshift128 { state1, state2 }
    }

    fn next(&mut self) {
        let s1: u64 = self.state1;
        let s0: u64 = self.state2;
        self.state1 = s0;
        self.state2 = s1 ^ (s1 << 23) ^ (s1 >> 17) ^ s0 ^ (s0 >> 26);
    }
}

fn main() {
    let mut sample: Xorshift128 = Xorshift128::new(12345,67890);

    println!("{sample:#?}")
}
