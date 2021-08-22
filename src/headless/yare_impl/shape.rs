use crate::base::{SPIRIT_COSTS_CIRCLE, SPIRIT_COSTS_SQUARE, SPIRIT_COSTS_TRIANGLE};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

impl From<usize> for Shape {
    fn from(shape: usize) -> Shape {
        match shape {
            0 => Shape::Circle,
            1 => Shape::Square,
            _ => Shape::Triangle,
        }
    }
}

impl Shape {
    pub fn spirit_cost(&self, current_count: usize) -> i32 {
        let costs = match self {
            Shape::Circle => SPIRIT_COSTS_CIRCLE,
            Shape::Square => SPIRIT_COSTS_SQUARE,
            Shape::Triangle => SPIRIT_COSTS_TRIANGLE,
        };
        costs
            .iter()
            .filter(|(threshold, _cost)| current_count >= *threshold as usize)
            .max_by_key(|(threshold, _cost)| threshold)
            .map(|(_threshold, cost)| *cost)
            .unwrap_or_default()
    }

    pub fn base_size(&self) -> i32 {
        match self {
            Shape::Circle => 1,
            Shape::Square => 10,
            Shape::Triangle => 3,
        }
    }
}
