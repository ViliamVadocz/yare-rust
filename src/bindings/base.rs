use crate::bindings::position::Position;

/// If an enemy spirit is within this radius to a base,
/// it will not produce new spirits.
pub const DISRUPTION_RADIUS: f32 = 400.;
/// (Threshold, energy cost) pairs for circle spirit production.
pub const SPIRIT_COSTS_CIRCLE: &[(u32, i32)] = &[(0, 25), (50, 50), (100, 90), (200, 150)];
/// (Threshold, energy cost) pairs for square spirit production.
pub const SPIRIT_COSTS_SQUARE: &[(u32, i32)] = &[(0, 360), (10, 500), (16, 700)];
/// (Threshold, energy cost) pairs for triangle spirit production.
pub const SPIRIT_COSTS_TRIANGLE: &[(u32, i32)] = &[(0, 90), (30, 160), (120, 300)];
/// Spirit production position offset from base.
/// Index of offset corresponds to player index.
pub const PRODUCTION_OFFSET: [Position; 2] =
    [Position { x: -20., y: -60. }, Position { x: 20., y: 60. }];
/// The hp the bases star with.
pub const START_BASE_HP: u32 = 8;

/// Spirit start offset from base for circles.
pub const CIRCLE_START_OFFSET: [&[Position]; 2] = [
    &[
        Position { x: -240., y: -100. },
        Position { x: -220., y: -100. },
        Position { x: -200., y: -100. },
        Position { x: -180., y: -100. },
        Position { x: -160., y: -100. },
        Position { x: -140., y: -100. },
        Position { x: -230., y: -80. },
        Position { x: -210., y: -80. },
        Position { x: -190., y: -80. },
        Position { x: -170., y: -80. },
        Position { x: -150., y: -80. },
        Position { x: -130., y: -80. },
    ],
    &[
        Position { x: 240., y: 100. },
        Position { x: 220., y: 100. },
        Position { x: 200., y: 100. },
        Position { x: 180., y: 100. },
        Position { x: 160., y: 100. },
        Position { x: 140., y: 100. },
        Position { x: 230., y: 80. },
        Position { x: 210., y: 80. },
        Position { x: 190., y: 80. },
        Position { x: 170., y: 80. },
        Position { x: 150., y: 80. },
        Position { x: 130., y: 80. },
    ],
];

/// Spirit start offset from base for squares.
pub const SQUARE_START_OFFSET: [&[Position]; 2] = [
    &[
        Position { x: -240., y: -100. },
        Position { x: -220., y: -100. },
        Position { x: -200., y: -100. },
    ],
    &[
        Position { x: 240., y: 100. },
        Position { x: 220., y: 100. },
        Position { x: 200., y: 100. },
    ],
];

/// Spirit start offset from base for triangles.
pub const TRIANGLE_START_OFFSET: [&[Position]; 2] = [
    &[
        Position { x: -240., y: -100. },
        Position { x: -220., y: -100. },
        Position { x: -200., y: -100. },
        Position { x: -180., y: -100. },
        Position { x: -160., y: -100. },
        Position { x: -140., y: -100. },
    ],
    &[
        Position { x: 240., y: 100. },
        Position { x: 220., y: 100. },
        Position { x: 200., y: 100. },
        Position { x: 180., y: 100. },
        Position { x: 160., y: 100. },
        Position { x: 140., y: 100. },
    ],
];

#[cfg(not(feature = "headless"))]
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
    /// See the `SPIRIT_COSTS` constants for each shape.
    #[link_name = "currentSpiritCost"]
    pub fn current_spirit_cost(index: usize) -> i32;

    /// Get the energy capacity of the base.
    /// This is 400 for circles, 1000 for squares, 600 for triangles.
    #[link_name = "energyCapacity"]
    pub fn energy_capacity(index: usize) -> i32;

    /// Get the current energy stored in the base.
    pub fn energy(index: usize) -> i32;

    /// Get the hp of the base.
    /// This is how many ticks of energizing it can survive when it has no
    /// energy to block.
    pub fn hp(index: usize) -> u32;

    /// Get the index of the player who owns the base.
    #[link_name = "playerId"]
    pub fn player_id(index: usize) -> usize;

    /// Get the position of the base.
    pub fn position(index: usize) -> Position;
}
