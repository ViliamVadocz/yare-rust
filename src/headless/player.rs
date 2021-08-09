use crate::headless::yare_impl::{ME, PLAYER_NUM};

pub unsafe fn count() -> usize {
    PLAYER_NUM
}
pub unsafe fn me() -> usize {
    ME
}

#[no_mangle]
pub unsafe extern "C" fn player_count() -> usize {
    PLAYER_NUM
}
#[no_mangle]
pub unsafe extern "C" fn player_me() -> usize {
    ME
}
