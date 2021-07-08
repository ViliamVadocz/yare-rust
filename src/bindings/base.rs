use crate::bindings::position::Position;

/// If an enemy spirit is within this radius to a base,
/// it will not produce new spirits.
pub const DISRUPTION_RADIUS: f32 = 400.;
/// (Threshold, energy cost) pairs for circle spirit production.
pub const SPIRIT_COSTS_CIRCLE: &[(u32, u32)] = &[(0, 25), (50, 50), (100, 100), (200, 200), (300, 400)];
/// (Threshold, energy cost) pairs for square spirit production.
pub const SPIRIT_COSTS_SQUARE: &[(u32, u32)] = &[(0, 400), (10, 800)];
/// (Threshold, energy cost) pairs for triangle spirit production.
pub const SPIRIT_COSTS_TRIANGLE: &[(u32, u32)] = &[(0, 60), (30, 120), (120, 300)];
/// Offset from the player 0 base to the spirit production position.
pub const PRODUCTION_OFFSET_0: Position = Position {x: -20., y: -60.};
/// Offset from the player 1 base to the spirit production position.
pub const PRODUCTION_OFFSET_1: Position = Position {x: 20., y: 60.};

#[link(wasm_import_module = "bases")]
extern "C" {
    /// Get the number of bases in the game.
    /// Index 0 is always your base.
    ///
    /// ### Usage
    /// ```
    /// for index in 0..unsafe { base::count() } {
    ///     // Do something for each base.
    /// }
    /// ```
    pub fn count() -> usize;

    /// Get the current cost to produce a new spirit.
    ///
    /// Each tick, if the energy exceeds the current spirit cost, a spirit will
    /// be generated and that much energy will be used up. The cost of a new
    /// spirit increases with the number of your spirits in the game.
    ///
    /// ### Circles
    /// ```
    /// | threshold | energy |
    /// |-----------|--------|
    /// | 0         | 25     |
    /// | 50        | 50     |
    /// | 100       | 100    |
    /// | 200       | 200    |
    /// | 300       | 400    |
    /// ```
    ///
    /// ### Squares
    /// ```
    /// | threshold | energy |
    /// |-----------|--------|
    /// | 0         | 400    |
    /// | 10        | 800    |
    /// ```
    ///
    /// ### Triangles
    /// ```
    /// | threshold | energy |
    /// |-----------|--------|
    /// | 0         | 60     |
    /// | 30        | 120    |
    /// | 120       | 300    |
    /// ```
    #[link_name = "currentSpiritCost"]
    pub fn current_spirit_cost(index: usize) -> u32;

    /// Get the energy capacity of the base.
    /// This is 400 for circles, 1000 for squares.
    #[link_name = "energyCapacity"]
    pub fn energy_capacity(index: usize) -> u32;

    /// Get the current energy stored in the base.
    pub fn energy(index: usize) -> u32;

    /// Get the hp of the base. Always 1.
    pub fn hp(index: usize) -> u32;

    /// Get the index of the player who owns the base.
    #[link_name = "playerId"]
    pub fn player_id(index: usize) -> usize;

    #[deprecated(note = "Use position instead")]
    /// Get the x coordinate of the base.
    #[link_name = "positionX"]
    pub fn position_x(index: usize) -> f32;

    #[deprecated(note = "Use position instead")]
    /// Get the y coordinate of the base.
    #[link_name = "positionY"]
    pub fn position_y(index: usize) -> f32;

    /// Get the position of the base.
    pub fn position(index: usize) -> Position;
}
