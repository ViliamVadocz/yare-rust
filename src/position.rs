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
pub struct Position {
    pub x: f32,
    pub y: f32,
}
