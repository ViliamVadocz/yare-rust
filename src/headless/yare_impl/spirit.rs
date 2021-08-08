use crate::{
    bindings::base::CIRCLE_START_OFFSET,
    yare_impl::{Pos, Shape},
};

#[derive(Clone, Debug)]
pub(crate) struct Spirit {
    pub energy_cap: i32,
    pub energy: i32,
    pub hp: u32,
    pub id: usize,
    pub player_id: usize,
    pub pos: Pos,
    pub shape: Shape,
    pub size: i32,
}

impl Spirit {
    pub fn new(player_id: usize, shape: Shape, pos: Pos, id: usize) -> Spirit {
        let size = match &shape {
            Shape::Circle => 1,
            Shape::Square => 10,
            Shape::Triangle => 3,
        };
        Spirit {
            energy_cap: size * 10,
            energy: size * 10,
            hp: 1,
            id, // TODO get id of last created spirit
            player_id,
            pos,
            shape,
            size,
        }
    }

    pub fn game_start(player_id: usize, shape: &Shape) -> Vec<Spirit> {
        // TODO Create spirits in starting positions.
        match shape {
            Shape::Circle => CIRCLE_START_OFFSET[player_id]
                .iter()
                .enumerate()
                .map(|(i, p)| Spirit::new(player_id, *shape, p.into(), i))
                .collect(),
            Shape::Square => vec![],
            Shape::Triangle => vec![],
        }
    }
}
