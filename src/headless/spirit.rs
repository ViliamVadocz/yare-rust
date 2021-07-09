use std::os::raw::c_char;

pub use crate::bindings::{
    position::Position,
    spirit::{
        ENERGIZE_RANGE,
        EXPLODE_DAMAGE,
        EXPLODE_RADIUS,
        JUMP_RANGE,
        MERGE_DISTANCE,
        MOVEMENT_SPEED,
    },
};
use crate::headless::yare_impl::{Command, Pos, COMMANDS, SPIRITS};

pub unsafe fn count() -> usize {
    SPIRITS.len()
}
pub unsafe fn divide(index: usize) {
    COMMANDS.push(Command::Divide { index });
}
pub unsafe fn energize_base(index: usize, base_index: usize) {
    COMMANDS.push(Command::EnergizeBase {
        index,
        target: base_index,
    });
}
pub unsafe fn energize_outpost(index: usize, outpost_index: usize) {
    COMMANDS.push(Command::EnergizeOutpost {
        index,
        target: outpost_index,
    });
}
pub unsafe fn energize(index: usize, spirit_index: usize) {
    COMMANDS.push(Command::Energize {
        index,
        target: spirit_index,
    });
}
pub unsafe fn energy_capacity(index: usize) -> u32 {
    SPIRITS[index].energy_cap
}
pub unsafe fn energy(index: usize) -> u32 {
    SPIRITS[index].energy
}
pub unsafe fn goto(index: usize, x: f32, y: f32) {
    COMMANDS.push(Command::Goto {
        index,
        target: Pos { x, y },
    })
}
pub unsafe fn hp(index: usize) -> u32 {
    SPIRITS[index].hp
}
pub unsafe fn id(index: usize) -> usize {
    SPIRITS[index].id
}
pub unsafe fn jump(index: usize, x: f32, y: f32) {
    COMMANDS.push(Command::Jump {
        index,
        target: Pos { x, y },
    });
}
pub unsafe fn merge(index: usize, spirit_index: usize) {
    COMMANDS.push(Command::Merge {
        index,
        target: spirit_index,
    });
}
pub unsafe fn player_id(index: usize) -> usize {
    SPIRITS[index].player_id
}
#[deprecated]
pub unsafe fn position_x(index: usize) -> f32 {
    SPIRITS[index].pos.x
}
#[deprecated]
pub unsafe fn position_y(index: usize) -> f32 {
    SPIRITS[index].pos.y
}
pub unsafe fn position(index: usize) -> Position {
    Position {
        x: SPIRITS[index].pos.x,
        y: SPIRITS[index].pos.y,
    }
}
pub unsafe fn shout(_index: usize, _string: *const c_char) {}
pub unsafe fn shape(index: usize) -> usize {
    SPIRITS[index].shape.into()
}
pub unsafe fn size(index: usize) -> u32 {
    SPIRITS[index].size
}
