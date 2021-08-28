use std::os::raw::c_char;

pub use crate::bindings::{
    id::Id,
    position::Position,
    spirit::{
        ENERGIZE_RANGE,
        EXPLODE_DAMAGE,
        EXPLODE_RADIUS,
        JUMP_COST_PER_DIST,
        MAX_CIRCLE_SIZE,
        MERGE_DISTANCE,
        MOVEMENT_SPEED,
    },
};
use crate::{
    get_static,
    push_static,
    yare_impl::{Command, Vec2, COMMANDS, SPIRITS},
};

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
pub unsafe fn energize_star(index: usize, star_index: usize) {
    push_static!(COMMANDS, Command::EnergizeStar {
        index,
        target: star_index,
    })
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
        target: Vec2 { x, y },
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
        target: Vec2 { x, y },
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

// uses u32's instead of usize to make node bindings more efficient
#[repr(C)]
pub struct ExternId {
    /// The player id. All spirits owned by a player have the same player id.
    pub player_id: u32,
    /// The spirit number.
    /// It is zero indexed, unlike in the game where it starts at 1.
    pub number: u32,
}

impl From<Id> for ExternId {
    fn from(id: Id) -> ExternId {
        ExternId {
            player_id: id.player_id as u32,
            number: id.number as u32,
        }
    }
}

#[no_mangle]
unsafe extern "C" fn spirit_count() -> u32 {
    count() as u32
}
#[no_mangle]
unsafe extern "C" fn spirit_divide(index: u32) {
    divide(index as usize);
}
#[no_mangle]
unsafe extern "C" fn spirit_energize_base(index: u32, base_index: u32) {
    energize_base(index as usize, base_index as usize)
}
#[no_mangle]
unsafe extern "C" fn spirit_energize_outpost(index: u32, outpost_index: u32) {
    energize_outpost(index as usize, outpost_index as usize)
}
#[no_mangle]
unsafe extern "C" fn spirit_energize(index: u32, spirit_index: u32) {
    energize(index as usize, spirit_index as usize)
}
#[no_mangle]
unsafe extern "C" fn spirit_energy_capacity(index: u32) -> i32 {
    energy_capacity(index as usize)
}
#[no_mangle]
unsafe extern "C" fn spirit_energy(index: u32) -> i32 {
    energy(index as usize)
}
#[no_mangle]
unsafe extern "C" fn spirit_explode(index: u32) {
    explode(index as usize)
}
#[no_mangle]
unsafe extern "C" fn spirit_goto(index: u32, x: f32, y: f32) {
    goto(index as usize, x, y)
}
#[no_mangle]
unsafe extern "C" fn spirit_hp(index: u32) -> u32 {
    hp(index as usize)
}
#[no_mangle]
unsafe extern "C" fn spirit_id(index: u32) -> ExternId {
    id(index as usize).into()
}
#[no_mangle]
unsafe extern "C" fn spirit_jump(index: u32, x: f32, y: f32) {
    jump(index as usize, x, y)
}
#[no_mangle]
unsafe extern "C" fn spirit_merge(index: u32, spirit_index: u32) {
    merge(index as usize, spirit_index as usize)
}
#[no_mangle]
unsafe extern "C" fn spirit_position(index: u32) -> Position {
    position(index as usize)
}
#[no_mangle]
unsafe extern "C" fn spirit_shout(_index: u32, _string: *const c_char) {}
#[no_mangle]
unsafe extern "C" fn spirit_shape(index: u32) -> u32 {
    shape(index as usize) as u32
}
#[no_mangle]
unsafe extern "C" fn spirit_size(index: u32) -> i32 {
    size(index as usize)
}
