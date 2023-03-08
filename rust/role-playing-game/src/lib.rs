// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn new(level: u32) -> Player {
        Self {
            health: 100,
            mana: match level >= 10 {
                true => Some(100),
                false => None
            },
            level,
        }
    }

    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player::new(self.level)),
            _ => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        return match self.mana {
            Some(mana) if mana >= mana_cost => {
                self.mana = Some(mana - mana_cost);
                mana_cost * 2
            }
            Some(mana) => 0,
            None => {
                self.health -= min(self.health, mana_cost);
                0
            }
        };
    }
}
