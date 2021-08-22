use super::vec2::Vec2;
use crate::bindings::outpost::{
    NORMAL_ATTACK,
    NORMAL_RANGE,
    UPGRADE_ATTACK,
    UPGRADE_ENERGY,
    UPGRADE_RANGE,
};

#[derive(Clone, Debug)]
pub(crate) struct Outpost {
    pub energy_cap: i32,
    pub energy: i32,
    pub player_id: usize,
    pub pos: Vec2,
}

impl Outpost {
    pub fn game_start() -> Vec<Outpost> {
        vec![Outpost {
            energy_cap: 1000,
            energy: 0,
            player_id: usize::MAX,
            pos: Vec2 { x: 2200., y: 1100. },
        }]
    }

    pub fn get_range(&self) -> f32 {
        if self.energy >= UPGRADE_ENERGY {
            UPGRADE_RANGE
        } else {
            NORMAL_RANGE
        }
    }

    pub fn get_attack_energy(&self) -> i32 {
        if self.energy >= UPGRADE_ENERGY {
            UPGRADE_ATTACK
        } else {
            NORMAL_ATTACK
        }
    }
}
