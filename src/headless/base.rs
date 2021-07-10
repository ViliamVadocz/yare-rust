pub use crate::bindings::{
    base::{
        DISRUPTION_RADIUS,
        PRODUCTION_OFFSET_0,
        PRODUCTION_OFFSET_1,
        SPIRIT_COSTS_CIRCLE,
        SPIRIT_COSTS_SQUARE,
        SPIRIT_COSTS_TRIANGLE,
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
#[deprecated]
pub unsafe fn position_x(index: usize) -> f32 {
    BASES[index].pos.x
}
#[deprecated]
pub unsafe fn position_y(index: usize) -> f32 {
    BASES[index].pos.y
}
pub unsafe fn position(index: usize) -> Position {
    Position {
        x: BASES[index].pos.x,
        y: BASES[index].pos.y,
    }
}
