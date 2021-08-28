use super::{base::Base, shape::Shape, vec2::Vec2};
use crate::bindings::{
    base::{CIRCLE_START_OFFSET, SQUARE_START_OFFSET, TRIANGLE_START_OFFSET},
    spirit::ENERGIZE_RANGE,
};

#[derive(Clone, Debug)]
pub(crate) struct Spirit {
    pub energy_cap: i32,
    pub energy: i32,
    pub hp: u32,
    pub id: usize,
    pub player_id: usize,
    pub pos: Vec2,
    pub shape: Shape,
    pub size: i32,
}

impl Spirit {
    pub fn new(player_id: usize, shape: Shape, pos: Vec2, id: usize) -> Spirit {
        let size = shape.base_size();
        Spirit {
            energy_cap: size * 10,
            energy: size * 10,
            hp: 1,
            id,
            player_id,
            pos,
            shape,
            size,
        }
    }

    pub fn game_start(player_id: usize, shape: &Shape) -> Vec<Spirit> {
        let base_pos = Base::base_pos(player_id);
        match shape {
            Shape::Circle => CIRCLE_START_OFFSET[player_id]
                .iter()
                .enumerate()
                .map(|(i, p)| Spirit::new(player_id, *shape, base_pos + p.into(), i))
                .collect(),
            Shape::Square => SQUARE_START_OFFSET[player_id]
                .iter()
                .enumerate()
                .map(|(i, p)| Spirit::new(player_id, *shape, base_pos + p.into(), i))
                .collect(),
            Shape::Triangle => TRIANGLE_START_OFFSET[player_id]
                .iter()
                .enumerate()
                .map(|(i, p)| Spirit::new(player_id, *shape, base_pos + p.into(), i))
                .collect(),
        }
    }

    pub fn energize_amount(&self) -> i32 {
        self.size.min(self.energy)
    }

    pub fn energize_self_amount(&self) -> i32 {
        self.size.min(self.energy_cap - self.energy)
    }

    pub fn can_energize(&self, player_id: usize, target_pos: Vec2) -> bool {
        !(self.hp < 1 || player_id != self.player_id || self.pos.dist(target_pos) > ENERGIZE_RANGE)
    }
}
