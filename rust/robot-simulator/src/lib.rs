use std::ops::Add;
// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
const DXDY: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
impl Direction {
    pub fn dxdy(&self) -> (i8, i8) {
        DXDY[*self as usize]
    }

    pub fn turn(&self, clockwise: bool) -> Direction {
        let discriminant = *self as i8;
        let new_discriminant = (discriminant + if clockwise { 1 } else { 3 }) % 4;
        // we did arithmetic modulo 4 so it is ok to use unsafe
        unsafe { std::mem::transmute(new_discriminant) }
    }
}

#[derive(Clone, Copy)]
struct X(i32);
impl Add<i32> for X {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        Self(self.0 + other)
    }
}

#[derive(Clone, Copy)]
struct Y(i32);
impl Add<i32> for Y {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        Self(self.0 + other)
    }
}

enum Instruction {
    TurnLeft,
    TurnRight,
    Advance,
}

impl TryFrom<char> for Instruction {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Self::TurnLeft),
            'R' => Ok(Self::TurnRight),
            'A' => Ok(Self::Advance),
            _ => Err(format!("Unexpected instructions: {c}")),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Robot {
    x: X,
    y: Y,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            x: X(x),
            y: Y(y),
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        Self {
            direction: self.direction.turn(true),
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        Self {
            direction: self.direction.turn(false),
            ..self
        }
    }

    pub fn advance(self) -> Self {
        Self {
            x: self.x + self.direction.dxdy().0.into(),
            y: self.y + self.direction.dxdy().1.into(),
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .map(|c| Instruction::try_from(c).unwrap())
            .fold(self, |robot, instruction| match instruction {
                Instruction::TurnLeft => robot.turn_left(),
                Instruction::TurnRight => robot.turn_right(),
                Instruction::Advance => robot.advance(),
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x.0, self.y.0)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
