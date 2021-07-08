use crate::headless::yare_impl::{PLAYER_NUM, ME};

pub unsafe fn count() -> usize {
    PLAYER_NUM
}
pub unsafe fn me() -> usize {
    ME
}
