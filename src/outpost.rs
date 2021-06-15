use crate::position::Position;

#[link(wasm_import_module = "outposts")]
extern "C" {
    pub fn count() -> usize;

    #[link_name = "energyCapacity"]
    pub fn energy_capacity(index: usize) -> u32;

    pub fn energy(index: usize) -> u32;

    pub fn player_id(index: usize) -> usize;

    #[link_name = "positionX"]
    pub fn position_x(index: usize) -> f32;

    #[link_name = "positionY"]
    pub fn position_y(index: usize) -> f32;

    pub fn position(index: usize) -> Position;

    pub fn range(index: usize) -> f32;

    pub fn size(index: usize) -> u32;
}
