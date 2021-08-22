pub use crate::bindings::{
    base::{
        CIRCLE_START_OFFSET,
        DISRUPTION_RADIUS,
        PRODUCTION_OFFSET,
        SPIRIT_COSTS_CIRCLE,
        SPIRIT_COSTS_SQUARE,
        SPIRIT_COSTS_TRIANGLE,
        SQUARE_START_OFFSET,
        START_BASE_HP,
        TRIANGLE_START_OFFSET,
    },
    position::Position,
};
use crate::{get_static, yare_impl::BASES};

pub unsafe fn count() -> usize {
    get_static!(BASES).len()
}
pub unsafe fn current_spirit_cost(index: usize) -> i32 {
    get_static!(BASES)[index].spirit_cost
}
pub unsafe fn energy_capacity(index: usize) -> i32 {
    get_static!(BASES)[index].energy_cap
}
pub unsafe fn energy(index: usize) -> i32 {
    get_static!(BASES)[index].energy
}
pub unsafe fn hp(index: usize) -> u32 {
    get_static!(BASES)[index].hp
}
pub unsafe fn player_id(index: usize) -> usize {
    get_static!(BASES)[index].player_id
}
pub unsafe fn position(index: usize) -> Position {
    Position {
        x: get_static!(BASES)[index].pos.x,
        y: get_static!(BASES)[index].pos.y,
    }
}

#[no_mangle]
unsafe extern "C" fn base_count() -> u32 {
    count() as u32
}
#[no_mangle]
unsafe extern "C" fn base_current_spirit_cost(index: u32) -> i32 {
    current_spirit_cost(index as usize)
}
#[no_mangle]
unsafe extern "C" fn base_energy_capacity(index: u32) -> i32 {
    energy_capacity(index as usize)
}
#[no_mangle]
unsafe extern "C" fn base_energy(index: u32) -> i32 {
    energy(index as usize)
}
#[no_mangle]
unsafe extern "C" fn base_hp(index: u32) -> u32 {
    hp(index as usize)
}
#[no_mangle]
unsafe extern "C" fn base_player_id(index: u32) -> u32 {
    player_id(index as usize) as u32
}
#[no_mangle]
unsafe extern "C" fn base_position(index: u32) -> Position {
    position(index as usize)
}
