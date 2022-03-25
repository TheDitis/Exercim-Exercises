use std::cmp::{Ordering};

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let mana = if self.level < 10 { None } else { Some(100) };
        match self.health {
            0 => Some(Player {  health: 100, mana, ..*self }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                match mana.cmp(&mana_cost) {
                    Ordering::Equal | Ordering::Greater => {
                        self.mana = Some(mana - mana_cost);
                        mana_cost * 2
                    },
                    Ordering::Less => 0,
                }
            },
            None => {
                self.health = if mana_cost > self.health { 0 } else { self.health - mana_cost };
                0
            },
        }
    }
}
