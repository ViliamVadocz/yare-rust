#![allow(clashing_extern_declarations)]

//! You should export a tick function with the following signature:
//! ```
//! #[no_mangle]
//! pub extern "C" fn tick(tick: u32) {}
//! ```

pub mod base;
pub mod console;
pub mod outpost;
pub mod player;
pub mod position;
pub mod spirit;
pub mod star;
