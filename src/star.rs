use crate::position::Position;

#[link(wasm_import_module = "stars")]
extern "C" {
    pub fn count() -> usize;

    #[link_name = "positionX"]
    pub fn position_x(index: usize) -> f32;

    #[link_name = "positionY"]
    pub fn position_y(index: usize) -> f32;

    pub fn position(index: usize) -> Position;
}
