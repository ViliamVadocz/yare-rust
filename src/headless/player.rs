use crate::headless::yare_impl::{ME, PLAYER_NUM};

pub unsafe fn count() -> usize {
    PLAYER_NUM
}
pub unsafe fn me() -> usize {
    ME
}
