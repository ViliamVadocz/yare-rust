use crate::bindings::id::Id;

use std::os::raw::c_char;

use crate::bindings::position::Position;


/// Maximum range of energy transfer.
pub const ENERGIZE_RANGE: f32 = 200.;
/// Movement speed of spirits in units per tick.
pub const MOVEMENT_SPEED: f32 = 20.;
/// Maximum jump distance for squares.
pub const JUMP_RANGE: f32 = 300.;
/// Maximum merge distance for circles. See `spirit::merge` for more
/// information.
pub const MERGE_DISTANCE: f32 = 12.;
/// Radius from the exploding spirit where enemy spirits take damage.
pub const EXPLODE_RADIUS: f32 = 100.;
/// The amount of damage given to enemy spirits in the `EXPLORE_RADIUS` once a spitit explodes.
pub const EXPLODE_DAMAGE: u32 = 10;

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
    /// *If spirit_index is the same as the index, it will instead attempt
    /// to harvest from the nearest star.
    pub fn energize(index: usize, spirit_index: usize);

    /// The energy capacity of a spirit is the maximum amount of energy is can
    /// hold. It is equal to ten times its size.
    #[link_name = "energyCapacity"]
    pub fn energy_capacity(index: usize) -> u32;

    /// Get the current energy of the spirit.
    pub fn energy(index: usize) -> u32;

    /// Spirit explodes, killing itself and dealing 10 damage
    /// to all enemy spirits within 100 radius.
    pub fn explode(index: usize);

    /// Moves spirit to target (x, y) with a speed of 20 units per tick.
    #[link_name = "move"]
    pub fn goto(index: usize, x: f32, y: f32);

    /// Get the spirit's hp.
    /// This is equal to 1 when the spirit is alive and 0 when it is dead.
    pub fn hp(index: usize) -> u32;

    /// Get the spirit's id. This is a combination of its player_id and number.
    pub fn id(index: usize) -> Id;

    /// Teleports the spirit into a new location up to 300 units away. Costs
    /// half of spirit's energy capacity.
    ///
    /// ### Disclaimer
    /// Only available for squares.
    pub fn jump(index: usize, x: f32, y: f32);

    /// Get the id of the last energized spirit by the spirit.
    /// If the last energized object was not a spirit, then blame @l0laapk3.
    #[link_name = "lastEnergizedId"]
    pub fn last_energized_id(index: usize) -> Id;

    /// Get the number of the last energized spirit by the spirit.
    /// If the last energized object was not a spirit, then blame @l0laapk3.
    #[link_name = "lastEnergizedNumber"]
    pub fn last_energized_number(index: usize) -> usize;

    /// Get the player id of the last energized spirit by the spirit.
    /// If the last energized object was not a spirit, then blame @l0laapk3.
    #[link_name = "lastEnergizedPlayerId"]
    pub fn last_energized_player_id(index: usize) -> usize;

    /// Merges the spirit into another friendly spirit. Target spirit
    /// needs to be within 12 units in both x and y.
    ///
    /// ### Disclaimer
    /// Only available for circles.
    pub fn merge(index: usize, spirit_index: usize);

    /// Get the number of the spirit.
    pub fn number(index: usize) -> usize;

    /// Get the index of the player who controls the spirit.
    #[link_name = "playerId"]
    pub fn player_id(index: usize) -> usize;

    #[deprecated(note = "Use position instead")]
    /// Get the x coordinate of the spirit.
    #[link_name = "positionX"]
    pub fn position_x(index: usize) -> f32;

    #[deprecated(note = "Use position instead")]
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