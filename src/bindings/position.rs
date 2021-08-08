/// A position in the 2D space.
/// This struct does not implement any traits because you should be
/// immediately translating it into your internal structures.
///
/// ### Usage
/// ```
/// let Position { x, y } = spirit::position(index);
/// let pos = MyVectorStruct { x, y };
/// ```
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Position {
    /// Horizontal coordinate.
    pub x: f32,
    /// Vertical coordinate.
    pub y: f32,
}
