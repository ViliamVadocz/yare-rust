#[cfg(not(feature = "headless"))]
use std::os::raw::c_char;

#[cfg(not(feature = "headless"))]
#[link(wasm_import_module = "console")]
extern "C" {
    /// Call `console.log`. This is shows in the console window above your code
    /// and is very useful for debugging.
    ///
    /// ### Usage
    /// ```
    /// use std::ffi::CString;
    /// fn log(string: &str) {
    ///     let c_string = CString::new(string).unwrap();
    ///     unsafe { console::log(c_string.as_ptr()) }
    /// }
    /// ```
    pub fn log(string: *const c_char);
}
