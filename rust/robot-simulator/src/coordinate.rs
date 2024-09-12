use std::ops::Add;

#[derive(Clone, Copy)]
pub struct X(pub i32);
impl Add<i32> for X {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        Self(self.0 + other)
    }
}

#[derive(Clone, Copy)]
pub struct Y(pub i32);
impl Add<i32> for Y {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        Self(self.0 + other)
    }
}
