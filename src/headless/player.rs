use crate::headless::yare_impl::{ME, PLAYER_NUM};
use crate::get_static;

pub unsafe fn count() -> usize {
    *get_static!(PLAYER_NUM)
}
pub unsafe fn me() -> usize {
    *get_static!(ME)
}

#[no_mangle]
pub unsafe extern "C" fn player_count() -> u32 {
    *get_static!(PLAYER_NUM) as u32
}
#[no_mangle]
pub unsafe extern "C" fn player_me() -> u32 {
    *get_static!(ME) as u32
}
