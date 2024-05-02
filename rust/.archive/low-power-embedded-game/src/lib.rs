// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let remainder: i16 = dividend % divisor;
    ((dividend - remainder) / divisor, remainder)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate()
        .filter(|(index, value)| divmod(*index as i16, 2).1 == 0)
        .map(|(index, value)| value)
}

pub struct Position(pub i16, pub i16);

impl Position {
    pub fn manhattan(&self) -> i16 {
        self.0.abs() + self.1.abs()
    }
}
