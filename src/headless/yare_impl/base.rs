use crate::yare_impl::{Shape, Pos};
use crate::bindings::base::{SPIRIT_COSTS_CIRCLE, SPIRIT_COSTS_SQUARE, SPIRIT_COSTS_TRIANGLE};

#[derive(Clone, Debug)]
pub(crate) struct Base {
    pub energy_cap: i32,
    pub energy: i32,
    pub hp: u32,
    pub player_id: usize,
    pub pos: Pos,
    pub spirit_cost: i32,
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
            pos: if player_id == 0 {
                Pos { x: 1600., y: 700. }
            } else {
                Pos { x: 2600., y: 1700. }
            },
            spirit_cost: match shape {
                Shape::Circle => SPIRIT_COSTS_CIRCLE[0].1,
                Shape::Square => SPIRIT_COSTS_SQUARE[0].1,
                Shape::Triangle => SPIRIT_COSTS_TRIANGLE[0].1,
            },
        }
    }
}