#[cfg(not(feature = "headless"))]
#[link(wasm_import_module = "random")]
extern "C" {
    /// Get a pseudorandom number in the range [0, 1)
    /// with an approximately uniform distribution.
    pub fn random() -> f32;
}
