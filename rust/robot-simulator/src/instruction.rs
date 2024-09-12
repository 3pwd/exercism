pub enum Instruction {
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
