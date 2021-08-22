#[cfg(not(feature = "headless"))]
#[link(wasm_import_module = "graphics")]
extern "C" {
    /// Set the colour of future shapes.
    #[link_name = "color"]
    pub fn colour(red: u8, green: u8, blue: u8, alpha: f32);

    /// Draw a circle with a centre at (x, y) and the given radius.
    pub fn circle(x: f32, y: f32, radius: f32);

    /// Draw a line from (x1, y1) to (x2, y2).
    pub fn line(x1: f32, y1: f32, x2: f32, y2: f32);

    /// Set the line width for future shapes.
    #[link_name = "lineWidth"]
    pub fn line_width(width: f32);

    /// Draw a rectable with the top left corner at (x, y)
    // and given width and height.
    pub fn rectangle(x: f32, y: f32, width: f32, height: f32);
}
