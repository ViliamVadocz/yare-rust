use std::os::raw::c_char;

#[link(wasm_import_module = "console")]
extern "C" {
    pub fn log(string: *const c_char);
}
