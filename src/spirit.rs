use crate::position::Position;

#[link(wasm_import_module = "spirits")]
extern "C" {
    pub fn count() -> usize;

    pub fn energize(index: usize, target_index: usize);

    #[link_name = "energizeBase"]
    pub fn energize_base(index: usize);

    #[link_name = "energizeEnemyBase"]
    pub fn energize_enemy_base(index: usize);

    #[link_name = "move"]
    pub fn goto(index: usize, x: f32, y: f32);

    pub fn merge(index: usize, target_index: usize);

    pub fn divide(index: usize);

    pub fn jump(index: usize, x: f32, y: f32);

    #[link_name = "isFriendly"]
    pub fn is_friendly(index: usize) -> bool;

    #[link_name = "positionX"]
    pub fn position_x(index: usize) -> f32;

    #[link_name = "positionY"]
    pub fn position_y(index: usize) -> f32;

    pub fn position(index: usize) -> Position;

    pub fn size(index: usize) -> f32;

    #[link_name = "energyCapacity"]
    pub fn energy_capacity(index: usize) -> f32;

    pub fn energy(index: usize) -> f32;

    pub fn hp(index: usize) -> f32;
}
