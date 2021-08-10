pub use crate::bindings::{position::Position, star::next_energy};
use crate::headless::yare_impl::STARS;
use crate::get_static;

pub unsafe fn active_at(index: usize) -> u32 {
    get_static!(STARS)[index].active_at
}
pub unsafe fn count() -> usize {
    get_static!(STARS).len()
}
pub unsafe fn energy_capacity(index: usize) -> i32 {
    get_static!(STARS)[index].energy_cap
}
pub unsafe fn energy(index: usize) -> i32 {
    get_static!(STARS)[index].energy
}
pub unsafe fn position(index: usize) -> Position {
    Position {
        x: get_static!(STARS)[index].pos.x,
        y: get_static!(STARS)[index].pos.y,
    }
}

#[no_mangle]
pub unsafe extern "C" fn star_active_at(index: usize) -> u32 {
    active_at(index)
}
#[no_mangle]
pub unsafe extern "C" fn star_count() -> usize {
    count()
}
#[no_mangle]
pub unsafe extern "C" fn star_energy_capacity(index: usize) -> i32 {
    energy_capacity(index)
}
#[no_mangle]
pub unsafe extern "C" fn star_energy(index: usize) -> i32 {
    energy(index)
}
#[no_mangle]
pub unsafe extern "C" fn star_position(index: usize) -> Position {
    position(index)
}
