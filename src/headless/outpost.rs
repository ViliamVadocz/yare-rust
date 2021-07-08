pub use crate::bindings::outpost::{UPGRADE_ENERGY, NORMAL_RANGE, UPGRADE_RANGE, NORMAL_ATTACK, UPGRADE_ATTACK};
use crate::headless::yare_impl::OUTPOSTS;

pub unsafe fn count() -> usize {
    OUTPOSTS.len()
}
pub unsafe fn energy_capacity(index: usize) -> u32 {
    OUTPOSTS[index].energy_cap
}
pub unsafe fn energy(index: usize) -> u32 {
    OUTPOSTS[index].energy
}
pub unsafe fn player_id(index: usize) -> usize {
    OUTPOSTS[index].player_id
}
pub unsafe fn position_x(index: usize) -> f32 {
    OUTPOSTS[index].pos.x
}
pub unsafe fn position_y(index: usize) -> f32 {
    OUTPOSTS[index].pos.y
}
pub unsafe fn range(index: usize) -> f32 {
    OUTPOSTS[index].range
}
