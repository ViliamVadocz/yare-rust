pub use crate::bindings::{position::Position, star::next_energy};
use crate::{get_static, yare_impl::STARS};

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
unsafe extern "C" fn star_active_at(index: u32) -> u32 {
    active_at(index as usize) as u32
}
#[no_mangle]
unsafe extern "C" fn star_count() -> u32 {
    count() as u32
}
#[no_mangle]
unsafe extern "C" fn star_energy_capacity(index: u32) -> i32 {
    energy_capacity(index as usize)
}
#[no_mangle]
unsafe extern "C" fn star_energy(index: u32) -> i32 {
    energy(index as usize)
}
#[no_mangle]
unsafe extern "C" fn star_position(index: u32) -> Position {
    position(index as usize)
}
