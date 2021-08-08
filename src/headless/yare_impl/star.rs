use crate::yare_impl::Pos;

#[derive(Clone, Debug)]
pub(crate) struct Star {
    pub energy_cap: i32,
    pub energy: i32,
    pub pos: Pos,
    pub active_at: u32,
}

impl Star {
    pub fn game_start() -> Vec<Star> {
        vec![
            Star {
                energy_cap: 1000,
                energy: 0,
                pos: Pos { x: 1000., y: 1000. },
                active_at: 0,
            },
            Star {
                energy_cap: 1000,
                energy: 0,
                pos: Pos { x: 2000., y: 1300. },
                active_at: 100,
            },
            Star {
                energy_cap: 1000,
                energy: 0,
                pos: Pos { x: 3200., y: 1400. },
                active_at: 0,
            },
        ]
    }
}
