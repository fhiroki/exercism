// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }
        let mut mana = Some(0);
        if self.level >= 10 {
            mana = Some(100);
        } else {
            mana = None;
        }
        Some(Player {
            health: 100,
            mana: mana,
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(x) => {
                if x < mana_cost {
                    return 0;
                }
                self.mana = Some(self.mana.unwrap() - mana_cost);
                return mana_cost * 2
            },
            None => {
                if (self.health >= mana_cost) {
                    self.health -= mana_cost;
                } else {
                    self.health = 0;
                }
                return 0;
            }
        }
    }
}
