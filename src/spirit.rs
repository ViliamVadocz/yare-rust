use std::os::raw::c_char;

use crate::position::Position;

/// Maximum range of energy transfer.
pub const ENERGIZE_RANGE: f32 = 200.;
/// Movement speed of spirits in units per tick.
pub const MOVEMENT_SPEED: f32 = 20.;
/// Maximum jump distance for squares.
pub const JUMP_RANGE: f32 = 300.;
/// Maximum merge distance for circles. See `spirit::merge` for more
/// information.
pub const MERGE_DISTANCE: f32 = 12.;

#[link(wasm_import_module = "spirits")]
extern "C" {
    /// Get the number of spirits in the game.
    ///
    /// ### Usage
    /// ```
    /// for index in 0..unsafe { spirit::count() } {
    ///     // Do something for each spirit.
    /// }
    /// ```
    pub fn count() -> usize;

    /// Divides spirit back into all the spirits that were merged into it.
    ///
    /// ### Disclaimer
    /// Only available for circles.
    pub fn divide(index: usize);

    /// Transfer energy to a base equal to the spirit's size.
    /// Maximum distance for energy transfer is 200 units.
    #[link_name = "energizeBase"]
    pub fn energize_base(index: usize, base_index: usize);

    /// Transfer energy to an outpost.
    /// Maximum distance for energy transfer is 200 units.
    #[link_name = "energizeOutpost"]
    pub fn energize_outpost(index: usize, outpost_index: usize);

    /// Transfer energy to another spirit* equal to the spirit's size.
    /// Maximum distance for energy transfer is 200 units.
    ///
    /// *If the target index is the same as the index, it will instead attempt
    /// to harvest from the nearest star.
    pub fn energize(index: usize, spirit_index: usize);

    /// The energy capacity of a spirit is the maximum amount of energy is can
    /// hold. It is equal to ten times its size.
    #[link_name = "energyCapacity"]
    pub fn energy_capacity(index: usize) -> u32;

    /// Get the current energy of the spirit.
    pub fn energy(index: usize) -> u32;

    /// Moves spirit to target (x, y) with a speed of 20 units per tick.
    #[link_name = "move"]
    pub fn goto(index: usize, x: f32, y: f32);

    /// Get the spirit's hp.
    /// This is equal to 1 when the spirit is alive and 0 when it is dead.
    pub fn hp(index: usize) -> u32;

    /// Get the spirit's id. This is persistent for the spirit, unlike its
    /// index. A player_id and id uniquely identify a spirit.
    pub fn id(index: usize) -> usize;

    /// Returns `true` if the spirit id matches the player id. This means you
    /// have control over the spirit.
    #[link_name = "isFriendly"]
    pub fn is_friendly(index: usize) -> bool;

    /// Teleports the spirit into a new location up to 300 units away. Costs
    /// half of spirit's energy capacity.
    ///
    /// ### Disclaimer
    /// Only available for squares.
    pub fn jump(index: usize, x: f32, y: f32);

    /// Merges the spirit into another friendly spirit. Target needs to be
    /// within 12 units in both x and y.
    ///
    /// ### Disclaimer
    /// Only available for circles.
    pub fn merge(index: usize, target_index: usize);

    /// Get the id of the player who controls the spirit.
    pub fn player_id(index: usize) -> usize;

    /// Get the x coordinate of the spirit.
    #[link_name = "positionX"]
    pub fn position_x(index: usize) -> f32;

    /// Get the y coordinate of the spirit.
    #[link_name = "positionY"]
    pub fn position_y(index: usize) -> f32;

    /// Get the position of the spirit.
    pub fn position(index: usize) -> Position;

    /// Shows a message above the spirit as a light-weight in-game
    /// communication. Useful for some debugging as well.
    ///
    /// ### Usage
    /// ```
    /// use std::ffi::CString;
    /// fn shout(spirit_index: usize, string: &str) {
    ///     let c_string = CString::new(string).unwrap();
    ///     unsafe { spirit::shout(spirit_index, c_string.as_ptr()) }
    /// }
    /// ```
    pub fn shout(index: usize, string: *const c_char);

    /// Get the shape of the spirit.
    /// This is 0 for circles, 1 for squares.
    pub fn shape(index: usize) -> usize;

    /// Get the size of the spirit. By default this is 1 for circles, 10 for
    /// squares. Circles can achieve a maximum size of 100 through merging.
    pub fn size(index: usize) -> u32;
}
