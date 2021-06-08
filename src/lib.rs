#![allow(clashing_extern_declarations)]

pub mod spirit {
    #[link(wasm_import_module = "spirits")]
    extern "C" {
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

        pub fn count() -> usize;

        #[link_name = "isFriendly"]
        pub fn is_friendly(index: usize) -> bool;

        #[link_name = "positionX"]
        pub fn position_x(index: usize) -> f32;

        #[link_name = "positionY"]
        pub fn position_y(index: usize) -> f32;

        pub fn size(index: usize) -> f32;

        #[link_name = "energyCapacity"]
        pub fn energy_capacity(index: usize) -> f32;

        pub fn energy(index: usize) -> f32;

        pub fn hp(index: usize) -> f32;
    }
}

pub mod base {
    #[link(wasm_import_module = "base")]
    extern "C" {
        #[link_name = "positionX"]
        pub fn position_x() -> f32;

        #[link_name = "positionY"]
        pub fn position_y() -> f32;

        pub fn size() -> f32;

        #[link_name = "energyCapacity"]
        pub fn energy_capacity() -> f32;

        pub fn energy() -> f32;

        pub fn hp() -> f32;
    }
}

pub mod enemy_base {
    #[link(wasm_import_module = "enemyBase")]
    extern "C" {
        #[link_name = "positionX"]
        pub fn position_x() -> f32;

        #[link_name = "positionY"]
        pub fn position_y() -> f32;

        pub fn size() -> f32;

        #[link_name = "energyCapacity"]
        pub fn energy_capacity() -> f32;

        pub fn energy() -> f32;

        pub fn hp() -> f32;
    }
}

pub mod star {
    #[link(wasm_import_module = "base")]
    extern "C" {
        pub fn count() -> usize;

        #[link_name = "positionX"]
        pub fn position_x(index: usize) -> f32;

        #[link_name = "positionY"]
        pub fn position_y(index: usize) -> f32;
    }
}

#[no_mangle]
pub extern "C" fn tick() {
    unsafe {
        let (x, y) = (base::position_x(), base::position_y());
        for index in 0..spirit::count() {
            if spirit::is_friendly(index) {
                spirit::goto(index, x, y)
            }
        }
    }
}
