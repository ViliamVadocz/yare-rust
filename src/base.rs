use crate::position::Position;

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

    /// Get the x coordinate of the base.
    #[link_name = "positionX"]
    pub fn position_x(index: usize) -> f32;

    /// Get the y coordinate of the base.
    #[link_name = "positionY"]
    pub fn position_y(index: usize) -> f32;

    /// Get the position of the base.
    pub fn position(index: usize) -> Position;

    /// Get the size of the base. Always 40.
    pub fn size(index: usize) -> u32;
}
