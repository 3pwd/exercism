use rand::{thread_rng, Rng};

trait Power {
    fn pow_by_squaring_modulo(self, exp: u64, modulo: u64) -> u64;
}

impl Power for u64 {
    fn pow_by_squaring_modulo(self, exp: u64, modulo: u64) -> u64 {
        let (mut base, mut exp, modulo) = (self as u128, exp as u128, modulo as u128);
        let mut result = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base % modulo;
            }
            exp >>= 1;
            base = base * base % modulo;
        }
        result as u64
    }
}

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.pow_by_squaring_modulo(a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    public_key(p, b_pub, a)
}
