#[derive(Clone, Copy, Debug)]
pub struct LinearCongruentialGenerator {
    state: i128,
    multiplier: i128,
    increment: i128,
    modulus: i128
}

impl LinearCongruentialGenerator {
   pub fn new(multiplier: i128, increment: i128, modulus: i128) -> Self {
        LinearCongruentialGenerator{state: 1, multiplier, increment, modulus}
    }
}
impl Iterator for LinearCongruentialGenerator {
    type Item = i128;

    fn next(&mut self) -> Option<i128> {
        // Using these variables to mimic the equation.
        let s = self.state;
        let a = self.multiplier;
        let b = self.increment;
        let m = self.modulus;

        let next_state = (a * s + b) % m;
        self.state = next_state;
        Some(next_state)
    }
}

