use crate::position::Position;

/// Energy needed for the outpost to "upgrade".
pub const UPGRADE_ENERGY: u32 = 500;
/// Range while the outpost has less energy than `UPGRADE_ENERGY`.
pub const NORMAL_RANGE: f32 = 400.;
/// Range while the outpost has at least `UPGRADE_ENERGY`.
pub const UPGRADE_RANGE: f32 = 500.;

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
    pub fn energy_capacity(index: usize) -> u32;

    /// Get the current energy stored in the outpost.
    pub fn energy(index: usize) -> u32;

    /// Get the id of the player who owns the outpost.
    pub fn player_id(index: usize) -> usize;

    /// Get the x coordinate of the outpost.
    #[link_name = "positionX"]
    pub fn position_x(index: usize) -> f32;

    /// Get the y coordinate of the outpost.
    #[link_name = "positionY"]
    pub fn position_y(index: usize) -> f32;

    /// Get the position of the outpost.
    pub fn position(index: usize) -> Position;

    /// Get the range of the outpost.
    pub fn range(index: usize) -> f32;

    /// Get the size of the outpost.
    pub fn size(index: usize) -> u32;
}
