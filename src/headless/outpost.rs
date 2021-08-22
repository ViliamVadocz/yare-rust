pub use crate::bindings::{
    outpost::{NORMAL_ATTACK, NORMAL_RANGE, UPGRADE_ATTACK, UPGRADE_ENERGY, UPGRADE_RANGE},
    position::Position,
};
use crate::{get_static, yare_impl::OUTPOSTS};

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
unsafe extern "C" fn outpost_count() -> u32 {
    count() as u32
}
#[no_mangle]
unsafe extern "C" fn outpost_energy_capacity(index: u32) -> i32 {
    energy_capacity(index as usize)
}
#[no_mangle]
unsafe extern "C" fn outpost_energy(index: u32) -> i32 {
    energy(index as usize)
}
#[no_mangle]
unsafe extern "C" fn outpost_player_id(index: u32) -> u32 {
    player_id(index as usize) as u32
}
#[no_mangle]
unsafe extern "C" fn outpost_position(index: u32) -> Position {
    position(index as usize)
}
#[no_mangle]
unsafe extern "C" fn outpost_range(index: u32) -> f32 {
    range(index as usize)
}
