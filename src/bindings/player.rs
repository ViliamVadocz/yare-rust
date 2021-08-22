#[cfg(not(feature = "headless"))]
#[link(wasm_import_module = "players")]
extern "C" {
    /// Get the number of players in the game.
    pub fn count() -> usize;

    /// Get your player index.
    /// This can be used to identify who controls things.
    pub fn me() -> usize;
}
