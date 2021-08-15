use crate::{
    bindings::base::{SPIRIT_COSTS_CIRCLE, SPIRIT_COSTS_SQUARE, SPIRIT_COSTS_TRIANGLE},
    yare_impl::{Pos, Shape},
};

#[derive(Clone, Debug)]
pub(crate) struct Base {
    pub energy_cap: i32,
    pub energy: i32,
    pub hp: u32,
    pub player_id: usize,
    pub pos: Pos,
    pub spirit_cost: i32,
    pub disrupted: bool,
}

impl Base {
    pub fn game_start(player_id: usize, shape: &Shape) -> Base {
        Base {
            energy_cap: match shape {
                Shape::Circle => 400,
                Shape::Square => 1000,
                Shape::Triangle => 600,
            },
            energy: 0,
            hp: 1,
            player_id,
            pos: Base::base_pos(player_id),
            spirit_cost: match shape {
                Shape::Circle => SPIRIT_COSTS_CIRCLE[0].1,
                Shape::Square => SPIRIT_COSTS_SQUARE[0].1,
                Shape::Triangle => SPIRIT_COSTS_TRIANGLE[0].1,
            },
            disrupted: false,
        }
    }

    pub fn base_pos(player_id: usize) -> Pos {
        if player_id == 0 {
            Pos { x: 1600., y: 700. }
        } else {
            Pos { x: 2600., y: 1700. }
        }
    }
}
