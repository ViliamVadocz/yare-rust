#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Circle,
    Square,
    Triangle,
}

impl From<Shape> for usize {
    fn from(shape: Shape) -> usize {
        match shape {
            Shape::Circle => 0,
            Shape::Square => 1,
            Shape::Triangle => 2,
        }
    }
}
