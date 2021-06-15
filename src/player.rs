#[link(wasm_import_module = "players")]
extern "C" {
    pub fn count() -> usize;
}
