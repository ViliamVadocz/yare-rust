pub use crate::bindings::base::DISRUPTION_RADIUS;
pub use crate::bindings::base::SPIRIT_COSTS_CIRCLE;
pub use crate::bindings::base::SPIRIT_COSTS_SQUARE;
use crate::headless::yare_impl::BASES;

pub unsafe fn count() -> usize {
    BASES.len()
}

pub unsafe fn current_spirit_cost(index: usize) -> u32 {
    BASES[index].spirit_cost
}

pub unsafe fn energy_capacity(index: usize) -> u32 {
    BASES[index].energy_cap
}

pub unsafe fn energy(index: usize) -> u32 {
    BASES[index].energy
}

pub unsafe fn hp(index: usize) -> u32 {
    BASES[index].hp
}

pub unsafe fn player_id(index: usize) -> usize {
    BASES[index].player_id
}

pub unsafe fn position_x(index: usize) -> f32 {
    BASES[index].pos.x
}

pub unsafe fn position_y(index: usize) -> f32 {
    BASES[index].pos.y
}
