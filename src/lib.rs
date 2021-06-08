#[link(wasm_import_module = "imports")]
extern "C" {
    pub fn energize(index: usize, target_index: usize);

    #[link_name = "move"]
    pub fn goto(index: usize, x: f32, y: f32);

    pub fn merge(index: usize, target_index: usize);

    pub fn divide(index: usize);

    pub fn jump(index: usize, x: f32, y: f32);

    #[link_name = "spiritCount"]
    pub fn spirit_count() -> usize;

    #[link_name = "spiritIsFriendly"]
    pub fn spirit_is_friendly(index: usize) -> bool;

    #[link_name = "spiritPositionX"]
    pub fn spirit_position_x(index: usize) -> f32;

    #[link_name = "spiritPositionY"]
    pub fn spirit_position_y(index: usize) -> f32;

    #[link_name = "spiritSize"]
    pub fn spirit_size(index: usize) -> f32;

    #[link_name = "spiritEnergyCapacity"]
    pub fn spirit_energy_capacity(index: usize) -> f32;

    #[link_name = "spiritEnergy"]
    pub fn spirit_energy(index: usize) -> f32;

    #[link_name = "spiritHp"]
    pub fn spirit_hp(index: usize) -> f32;

    #[link_name = "basePositionX"]
    pub fn base_position_x() -> f32;

    #[link_name = "basePositionY"]
    pub fn base_position_y() -> f32;

    #[link_name = "baseSize"]
    pub fn base_size() -> f32;

    #[link_name = "baseEnergyCapacity"]
    pub fn base_energy_capacity() -> f32;

    #[link_name = "baseEnergy"]
    pub fn base_energy() -> f32;

    #[link_name = "baseHp"]
    pub fn base_hp() -> f32;

    #[link_name = "enemyBasePositionX"]
    pub fn enemy_base_position_x() -> f32;

    #[link_name = "enemyBasePositionY"]
    pub fn enemy_base_position_y() -> f32;

    #[link_name = "enemyBaseSize"]
    pub fn enemy_base_size() -> f32;

    #[link_name = "enemyBaseEnergyCapacity"]
    pub fn enemy_base_energy_capacity() -> f32;

    #[link_name = "enemyBaseEnergy"]
    pub fn enemy_base_energy() -> f32;

    #[link_name = "enemyBaseHp"]
    pub fn enemy_base_hp() -> f32;

    #[link_name = "starCount"]
    pub fn star_count() -> usize;

    #[link_name = "spiritPositionX"]
    pub fn star_position_x(index: usize) -> f32;

    #[link_name = "spiritPositionY"]
    pub fn star_position_y(index: usize) -> f32;
}

#[no_mangle]
pub extern "C" fn tick() {
    unsafe {
        let (x, y) = (base_position_x(), base_position_y());
        for index in 0..spirit_count() {
            if spirit_is_friendly(index) {
                goto(index, x, y)
            }
        }
    }
}
