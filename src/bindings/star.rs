use std::cmp::max;

use crate::bindings::position::Position;

/// Each tick a star generates 3 + 1% of its current energy,
/// rounded to the nearest integer.
pub fn next_energy(energy: i32) -> i32 {
    ((max(energy, 0) + 3) as f32 * 1.01).round() as i32
}

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

    #[deprecated(note = "Use position instead")]
    /// Get the x coordinate of the star.
    #[link_name = "positionX"]
    pub fn position_x(index: usize) -> f32;

    #[deprecated(note = "Use position instead")]
    /// Get the y coordinate of the star.
    #[link_name = "positionY"]
    pub fn position_y(index: usize) -> f32;

    /// Get the position of the star.
    pub fn position(index: usize) -> Position;
}
