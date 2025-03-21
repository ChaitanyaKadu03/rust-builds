#[derive(Debug)]
struct Xorshift128 {
    state1: u64,
    state2: u64,
}

impl Xorshift128 {
    fn new(state1: u64, state2: u64) -> Self {
        Xorshift128 { state1, state2 }
    }

    fn next(&mut self) -> f64 {
        let mut s1: u64 = self.state1;
        let s2: u64 = self.state2;

        self.state1 = s2;

        s1 ^= s1 << 23;
        s1 ^= s1 >> 17;
        s1 ^= s2;
        s1 ^= s2 >> 26;

        self.state2 = s1;

        let value: u64 = self.state1;
        let mut value: f64 = (value >> 11) as f64; // float value
        value = value / (1u64 << 53) as f64; // normalization
        1f64 - value
    }

    fn get_state(calculations: Vec<f64>) -> Option<Xorshift128> {
        // for value in calculations {
        //     let mut _value: f64 = value + 1f64;
        //     _value = value * (1u64 )
        // }

        None
    }
}

fn main() {
    let mut state: Xorshift128 = Xorshift128::new(1233744567890, 3646874774744);
}
