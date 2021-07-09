/// Id is a combination of player id and number.
/// This uniquely identifies a spirit.
#[repr(C)]
pub struct Id {
    /// The player id. All spirits owned by a player have the same player id.
    pub player_id: usize,
    /// The spirit number.
    /// It is zero indexed, unlike in the game where it starts at 1.
    pub number: usize,
}
