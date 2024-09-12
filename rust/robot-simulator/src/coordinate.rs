use std::ops::Add;

macro_rules! coordinate {
    ($name:ident) => {
        #[derive(Clone, Copy)]
        pub struct $name(pub i32);

        impl Add<i32> for $name {
            type Output = Self;

            fn add(self, other: i32) -> Self {
                Self(self.0 + other)
            }
        }
    };
}

coordinate!(X);
coordinate!(Y);
