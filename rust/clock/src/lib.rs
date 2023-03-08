extern crate core;

use core::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: (hours * 60 + minutes).rem_euclid(1440) // 24 * 60
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }

    pub fn hours(&self) ->i32 {
      self.minutes.div_euclid(60).rem_euclid(24)
    }

    pub fn minutes(&self) -> i32 {
        self.minutes.rem_euclid(60)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:02}:{:02}", self.hours(), self.minutes()))
    }
}

impl From<String> for Clock {
    fn from(clock: String) -> Self {
        let v: Vec<_> = clock
            .split(":")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();

        match v[..] {
            [hours, minutes] => Clock::new(hours, minutes),
            _ => panic!("cannot build Clock from this string"),
        }
    }
}
