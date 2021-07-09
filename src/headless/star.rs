pub use crate::bindings::{position::Position, star::next_energy};
use crate::headless::yare_impl::STARS;

pub unsafe fn active_at(index: usize) -> u32 {
    STARS[index].active_at
}
pub unsafe fn count() -> usize {
    STARS.len()
}
pub unsafe fn energy_capacity(index: usize) -> u32 {
    STARS[index].energy_cap
}
pub unsafe fn energy(index: usize) -> u32 {
    STARS[index].energy
}
#[deprecated]
pub unsafe fn position_x(index: usize) -> f32 {
    STARS[index].pos.x
}
#[deprecated]
pub unsafe fn position_y(index: usize) -> f32 {
    STARS[index].pos.y
}
pub unsafe fn position(index: usize) -> Position {
    Position {
        x: STARS[index].pos.x,
        y: STARS[index].pos.y,
    }
}
