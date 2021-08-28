use std::cmp::max;

#[cfg(not(feature = "headless"))]
use crate::bindings::position::Position;

/// Each tick a star generates 2 + 2% of its current energy,
/// rounded to the nearest integer.
pub fn next_energy(energy: i32) -> i32 {
    ((max(energy, 0) + 2) as f32 * 1.02).round() as i32
}

#[cfg(not(feature = "headless"))]
#[link(wasm_import_module = "stars")]
extern "C" {
    /// The tick at which a star begins to generate energy.
    #[link_name = "activeAt"]
    pub fn active_at(index: usize) -> u32;

    /// Get the number of stars in the game.
    ///
    /// ### Usage
    /// ```
    /// for index in 0..unsafe { star::count() } {
    ///     // Do something for each star.
    /// }
    /// ```
    pub fn count() -> usize;

    /// Get the energy capacity of the star.
    #[link_name = "energyCapacity"]
    pub fn energy_capacity(index: usize) -> i32;

    /// Get the current energy of the star.
    pub fn energy(index: usize) -> i32;

    /// Get the position of the star.
    pub fn position(index: usize) -> Position;
}
