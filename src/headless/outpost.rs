pub use crate::bindings::{
    outpost::{NORMAL_ATTACK, NORMAL_RANGE, UPGRADE_ATTACK, UPGRADE_ENERGY, UPGRADE_RANGE},
    position::Position,
};
use crate::headless::yare_impl::OUTPOSTS;

pub unsafe fn count() -> usize {
    OUTPOSTS.len()
}
pub unsafe fn energy_capacity(index: usize) -> i32 {
    OUTPOSTS[index].energy_cap
}
pub unsafe fn energy(index: usize) -> i32 {
    OUTPOSTS[index].energy
}
pub unsafe fn player_id(index: usize) -> usize {
    OUTPOSTS[index].player_id
}
pub unsafe fn position(index: usize) -> Position {
    Position {
        x: OUTPOSTS[index].pos.x,
        y: OUTPOSTS[index].pos.y,
    }
}
pub unsafe fn range(index: usize) -> f32 {
    OUTPOSTS[index].range
}
