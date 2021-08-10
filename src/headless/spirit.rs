use std::os::raw::c_char;

pub use crate::bindings::{
    id::Id,
    position::Position,
    spirit::{
        ENERGIZE_RANGE,
        EXPLODE_DAMAGE,
        EXPLODE_RADIUS,
        JUMP_COST_PER_DIST,
        MERGE_DISTANCE,
        MOVEMENT_SPEED,
    },
};
use crate::headless::yare_impl::{Command, Pos, COMMANDS, SPIRITS};

use crate::get_static;
use crate::push_static;

pub unsafe fn count() -> usize {
    get_static!(SPIRITS).len()
}
pub unsafe fn divide(index: usize) {
    push_static!(COMMANDS, Command::Divide { index });
}
pub unsafe fn energize_base(index: usize, base_index: usize) {
    push_static!(COMMANDS, Command::EnergizeBase {
        index,
        target: base_index,
    });
}
pub unsafe fn energize_outpost(index: usize, outpost_index: usize) {
    push_static!(COMMANDS, Command::EnergizeOutpost {
        index,
        target: outpost_index,
    });
}
pub unsafe fn energize(index: usize, spirit_index: usize) {
    push_static!(COMMANDS, Command::Energize {
        index,
        target: spirit_index,
    });
}
pub unsafe fn energy_capacity(index: usize) -> i32 {
    get_static!(SPIRITS)[index].energy_cap
}
pub unsafe fn energy(index: usize) -> i32 {
    get_static!(SPIRITS)[index].energy
}
pub unsafe fn explode(index: usize) {
    push_static!(COMMANDS, Command::Explode { index })
}
pub unsafe fn goto(index: usize, x: f32, y: f32) {
    push_static!(COMMANDS, Command::Goto {
        index,
        target: Pos { x, y },
    })
}
pub unsafe fn hp(index: usize) -> u32 {
    get_static!(SPIRITS)[index].hp
}
pub unsafe fn id(index: usize) -> Id {
    Id {
        player_id: get_static!(SPIRITS)[index].player_id,
        number: get_static!(SPIRITS)[index].id,
    }
}
pub unsafe fn jump(index: usize, x: f32, y: f32) {
    push_static!(COMMANDS, Command::Jump {
        index,
        target: Pos { x, y },
    });
}
pub unsafe fn merge(index: usize, spirit_index: usize) {
    push_static!(COMMANDS, Command::Merge {
        index,
        target: spirit_index,
    });
}
pub unsafe fn position(index: usize) -> Position {
    Position {
        x: get_static!(SPIRITS)[index].pos.x,
        y: get_static!(SPIRITS)[index].pos.y,
    }
}
pub unsafe fn shout(_index: usize, _string: *const c_char) {}
pub unsafe fn shape(index: usize) -> usize {
    get_static!(SPIRITS)[index].shape.into()
}
pub unsafe fn size(index: usize) -> i32 {
    get_static!(SPIRITS)[index].size
}

#[no_mangle]
pub unsafe extern "C" fn spirit_count() -> usize {
    count()
}
#[no_mangle]
pub unsafe extern "C" fn spirit_divide(index: usize) {
    divide(index);
}
#[no_mangle]
pub unsafe extern "C" fn spirit_energize_base(index: usize, base_index: usize) {
    energize_base(index, base_index)
}
#[no_mangle]
pub unsafe extern "C" fn spirit_energize_outpost(index: usize, outpost_index: usize) {
    energize_outpost(index, outpost_index)
}
#[no_mangle]
pub unsafe extern "C" fn spirit_energize(index: usize, spirit_index: usize) {
    energize(index, spirit_index)
}
#[no_mangle]
pub unsafe extern "C" fn spirit_energy_capacity(index: usize) -> i32 {
    energy_capacity(index)
}
#[no_mangle]
pub unsafe extern "C" fn spirit_energy(index: usize) -> i32 {
    energy(index)
}
#[no_mangle]
pub unsafe extern "C" fn spirit_explode(index: usize) {
    explode(index)
}
#[no_mangle]
pub unsafe extern "C" fn spirit_goto(index: usize, x: f32, y: f32) {
    goto(index, x, y)
}
#[no_mangle]
pub unsafe extern "C" fn spirit_hp(index: usize) -> u32 {
    hp(index)
}
#[no_mangle]
pub unsafe extern "C" fn spirit_id(index: usize) -> Id {
    id(index)
}
#[no_mangle]
pub unsafe extern "C" fn spirit_jump(index: usize, x: f32, y: f32) {
    jump(index, x, y)
}
#[no_mangle]
pub unsafe extern "C" fn spirit_merge(index: usize, spirit_index: usize) {
    merge(index, spirit_index)
}
#[no_mangle]
pub unsafe extern "C" fn spirit_position(index: usize) -> Position {
    position(index)
}
#[no_mangle]
pub unsafe extern "C" fn spirit_shout(_index: usize, _string: *const c_char) {}
#[no_mangle]
pub unsafe extern "C" fn spirit_shape(index: usize) -> usize {
    shape(index)
}
#[no_mangle]
pub unsafe extern "C" fn spirit_size(index: usize) -> i32 {
    size(index)
}
