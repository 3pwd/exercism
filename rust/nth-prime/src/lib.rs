struct Primes {
    primes: Vec<u32>,
    next_candidate: u32,
}

impl Primes {
    fn new() -> Self {
        Self {
            primes: Vec::new(),
            next_candidate: 2,
        }
    }

    fn is_prime(&self, n: u32) -> bool {
        let sqrt_n = (n as f32).sqrt() as u32;
        self.primes
            .iter()
            // it is enough to check divisibility by primes up to sqrt(n)
            .take_while(|&&p| p <= sqrt_n)
            .all(|&p| n % p != 0)
    }

    fn inc(&mut self) {
        self.next_candidate = match self.next_candidate {
            2 => 3,
            3 => 5,
            // we iterate alternatively in steps of 4 and 2 because we can skip multiples of 2 and 3
            // this makes the algorithm much faster (x15 speedup on my machine)
            _ => self.next_candidate + if self.next_candidate % 6 == 1 { 4 } else { 2 },
        }
    }

    fn push_and_inc(&mut self) -> u32 {
        let prime = self.next_candidate;
        self.primes.push(prime);
        self.inc();
        prime
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // we keep incrementing the candidate until we find a prime
        while !self.is_prime(self.next_candidate) {
            self.inc()
        }

        // then we store it into the cache, increment the candidate and return the prime
        Some(self.push_and_inc())
    }
}

pub fn nth(n: u32) -> u32 {
    Primes::new().nth(n as usize).unwrap()
}
