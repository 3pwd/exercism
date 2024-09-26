const FRAMES: u8 = 10;
const PINS: u8 = 10;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Default)]
pub struct BowlingGame {
    rolls: Vec<u16>,
    prev: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins + self.prev > PINS.into() {
            Err(Error::NotEnoughPinsLeft)
        } else if self.score().is_some() {
            Err(Error::GameComplete)
        } else {
            self.rolls.push(pins);
            self.prev = if self.prev + pins == 10 { 0 } else { pins };
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        let mut score = 0;
        let mut frame = 0;

        for _ in 1..=FRAMES {
            let first = self.rolls.get(frame)?;
            let second = self.rolls.get(frame + 1)?;

            score += first + second;
            if first + second >= 10 {
                let third = self.rolls.get(frame + 2)?;
                score += third;
            }
            // for a strike, we increment by one in order to count the next 2 rolls twice: as bonus points for the strike and as part of the next frame
            frame += if *first == 10 { 1 } else { 2 };
        }
        Some(score)
    }
}
