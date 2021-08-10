pub use crate::bindings::{
    outpost::{NORMAL_ATTACK, NORMAL_RANGE, UPGRADE_ATTACK, UPGRADE_ENERGY, UPGRADE_RANGE},
    position::Position,
};

#[macro_use]
use crate::get_static;

use crate::headless::yare_impl::OUTPOSTS;

pub unsafe fn count() -> usize {
    get_static!(OUTPOSTS).len()
}
pub unsafe fn energy_capacity(index: usize) -> i32 {
    get_static!(OUTPOSTS)[index].energy_cap
}
pub unsafe fn energy(index: usize) -> i32 {
    get_static!(OUTPOSTS)[index].energy
}
pub unsafe fn player_id(index: usize) -> usize {
    get_static!(OUTPOSTS)[index].player_id
}
pub unsafe fn position(index: usize) -> Position {
    Position {
        x: get_static!(OUTPOSTS)[index].pos.x,
        y: get_static!(OUTPOSTS)[index].pos.y,
    }
}
pub unsafe fn range(index: usize) -> f32 {
    get_static!(OUTPOSTS)[index].get_range()
}

#[no_mangle]
pub unsafe extern "C" fn outpost_count() -> usize {
    count()
}
#[no_mangle]
pub unsafe extern "C" fn outpost_energy_capacity(index: usize) -> i32 {
    energy_capacity(index)
}
#[no_mangle]
pub unsafe extern "C" fn outpost_energy(index: usize) -> i32 {
    energy(index)
}
#[no_mangle]
pub unsafe extern "C" fn outpost_player_id(index: usize) -> usize {
    player_id(index)
}
#[no_mangle]
pub unsafe extern "C" fn outpost_position(index: usize) -> Position {
    position(index)
}
#[no_mangle]
pub unsafe extern "C" fn outpost_range(index: usize) -> f32 {
    range(index)
}
