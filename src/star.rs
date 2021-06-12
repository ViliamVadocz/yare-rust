use crate::position::Position;

#[link(wasm_import_module = "stars")]
extern "C" {
    /// Get the number of stars in the game.
    ///
    /// ### Usage
    /// ```
    /// for index in 0..unsafe { star::count() } {
    ///     // Do something for each star.
    /// }
    /// ```
    pub fn count() -> usize;

    /// Get the x coordinate of the star.
    #[link_name = "positionX"]
    pub fn position_x(index: usize) -> f32;

    /// Get the y coordinate of the star.
    #[link_name = "positionY"]
    pub fn position_y(index: usize) -> f32;

    /// Get the position of the star.
    pub fn position(index: usize) -> Position;
}
