use crate::position::Position;

#[link(wasm_import_module = "bases")]
extern "C" {
    pub fn count() -> usize;

    #[link_name = "currentSpiritCost"]
    pub fn current_spirit_cost(index: usize) -> u32;

    #[link_name = "energyCapacity"]
    pub fn energy_capacity(index: usize) -> u32;

    pub fn energy(index: usize) -> u32;

    pub fn hp(index: usize) -> u32;

    #[link_name = "positionX"]
    pub fn position_x(index: usize) -> f32;

    #[link_name = "positionY"]
    pub fn position_y(index: usize) -> f32;

    pub fn position(index: usize) -> Position;

    pub fn size(index: usize) -> u32;
}
