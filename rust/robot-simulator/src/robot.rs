use crate::{
    coordinate::{X, Y},
    Direction, Instruction,
};
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
