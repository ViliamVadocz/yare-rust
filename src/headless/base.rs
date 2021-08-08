pub use crate::bindings::{
    base::{
        CIRCLE_START_OFFSET, DISRUPTION_RADIUS, PRODUCTION_OFFSET, SPIRIT_COSTS_CIRCLE,
        SPIRIT_COSTS_SQUARE, SPIRIT_COSTS_TRIANGLE, START_BASE_HP,
    },
    position::Position,
};
use crate::headless::yare_impl::BASES;

pub unsafe fn count() -> usize {
    BASES.len()
}
pub unsafe fn current_spirit_cost(index: usize) -> i32 {
    BASES[index].spirit_cost
}
pub unsafe fn energy_capacity(index: usize) -> i32 {
    BASES[index].energy_cap
}
pub unsafe fn energy(index: usize) -> i32 {
    BASES[index].energy
}
pub unsafe fn hp(index: usize) -> u32 {
    BASES[index].hp
}
pub unsafe fn player_id(index: usize) -> usize {
    BASES[index].player_id
}
pub unsafe fn position(index: usize) -> Position {
    Position {
        x: BASES[index].pos.x,
        y: BASES[index].pos.y,
    }
}
