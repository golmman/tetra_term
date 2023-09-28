use std::time::SystemTime;
use std::time::UNIX_EPOCH;

// https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use
const A: u64 = 48271;
const C: u64 = 0;
const M: u64 = 2147483647;

pub struct Random {
    seed: u64,
}

impl Random {
    pub fn new() -> Self {
        Self {
            seed: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
                % M,
        }
    }

    pub fn next(&mut self) -> u32 {
        self.seed = (A * self.seed + C) % M;
        self.seed as u32
    }
}
