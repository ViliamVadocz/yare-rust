use crate::yare_impl::Pos;
use crate::bindings::outpost::NORMAL_RANGE;

#[derive(Clone, Debug)]
pub(crate) struct Outpost {
    pub energy_cap: i32,
    pub energy: i32,
    pub player_id: usize,
    pub pos: Pos,
    pub range: f32,
}

impl Outpost {
    pub fn game_start() -> Vec<Outpost> {
        vec![Outpost {
            energy_cap: 1000,
            energy: 0,
            player_id: usize::MAX,
            pos: Pos { x: 2200., y: 1100. },
            range: NORMAL_RANGE,
        }]
    }
}
