struct PrimeFactors {
    current_factor: u64,
    n: u64,
}

impl PrimeFactors {
    pub fn new(n: u64) -> Self {
        Self {
            current_factor: 2,
            n,
        }
    }
}

impl Iterator for PrimeFactors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.n > 1 {
            if self.n % self.current_factor == 0 {
                self.n = self.n / self.current_factor;
                return Some(self.current_factor);
            }
            self.current_factor += 1;
        }
        None
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    PrimeFactors::new(n).collect()
}
