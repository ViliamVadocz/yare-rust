pub use crate::bindings::star::{SMALL_START_TICK, next_energy};
use crate::headless::yare_impl::STARS;

pub unsafe fn count() -> usize {
    STARS.len()
}
pub unsafe fn energy_capacity(index: usize) -> u32 {
    STARS[index].energy_cap
}
pub unsafe fn energy(index: usize) -> u32 {
    STARS[index].energy
}
pub unsafe fn position_x(index: usize) -> f32 {
    STARS[index].pos.x
}
pub unsafe fn position_y(index: usize) -> f32 {
    STARS[index].pos.y
}
