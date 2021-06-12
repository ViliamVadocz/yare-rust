use std::os::raw::c_char;

use crate::position::Position;

#[link(wasm_import_module = "spirits")]
extern "C" {
    pub fn count() -> usize;

    pub fn divide(index: usize);

    #[link_name = "energizeBase"]
    pub fn energize_base(index: usize, base_index: usize);

    pub fn energize(index: usize, target_index: usize);

    #[link_name = "energyCapacity"]
    pub fn energy_capacity(index: usize) -> u32;

    pub fn energy(index: usize) -> u32;

    #[link_name = "move"]
    pub fn goto(index: usize, x: f32, y: f32);

    pub fn hp(index: usize) -> u32;

    #[link_name = "isFriendly"]
    pub fn is_friendly(index: usize) -> bool;

    pub fn jump(index: usize, x: f32, y: f32);

    pub fn merge(index: usize, target_index: usize);

    #[link_name = "positionX"]
    pub fn position_x(index: usize) -> f32;

    #[link_name = "positionY"]
    pub fn position_y(index: usize) -> f32;

    pub fn position(index: usize) -> Position;

    pub fn shout(index: usize, string: *const c_char);

    pub fn shape(index: usize) -> usize;

    pub fn size(index: usize) -> u32;
}
