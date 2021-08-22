#[cfg(not(feature = "headless"))]
use crate::bindings::position::Position;

/// Energy needed for the outpost to "upgrade".
pub const UPGRADE_ENERGY: i32 = 500;
/// Range while the outpost has less energy than `UPGRADE_ENERGY`.
pub const NORMAL_RANGE: f32 = 400.;
/// Range while the outpost has at least `UPGRADE_ENERGY`.
pub const UPGRADE_RANGE: f32 = 600.;
/// Energy used for attacks while the outpost has less energy than
/// `UPGRADE_ENERGY`.
pub const NORMAL_ATTACK: i32 = 1;
/// Energy used for attacks while the outpost has at least `UPGRADE_ENERGY`.
pub const UPGRADE_ATTACK: i32 = 4;

#[cfg(not(feature = "headless"))]
#[link(wasm_import_module = "outposts")]
extern "C" {
    /// Get the number of outposts in the game.
    ///
    /// ### Usage
    /// ```
    /// for index in 0..unsafe { outpost::count() } {
    ///     // Do something for each outpost.
    /// }
    /// ```
    pub fn count() -> usize;

    /// Get the energy capacity of the outpost. Always 1000.
    #[link_name = "energyCapacity"]
    pub fn energy_capacity(index: usize) -> i32;

    /// Get the current energy stored in the outpost.
    pub fn energy(index: usize) -> i32;

    /// Get the index of the player who owns the outpost.
    #[link_name = "controlledBy"]
    pub fn player_id(index: usize) -> usize;

    /// Get the position of the outpost.
    pub fn position(index: usize) -> Position;

    /// Get the range of the outpost.
    pub fn range(index: usize) -> f32;
}
