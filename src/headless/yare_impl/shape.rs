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

impl Shape {
    pub fn spirit_cost(&self, current_count: usize) -> i32 {
        let costs = match self {
            Shape::Circle => SPIRIT_COSTS_CIRCLE,
            Shape::Square => SPIRIT_COSTS_SQUARE,
            Shape::Triangle => SPIRIT_COSTS_TRIANGLE,
        };
        let mut cost = costs[0].1;
        for i in 1..costs.len() {
            if current_count >= costs[i].0 as usize {
                cost = costs[i].1;
            }
        }
        cost
    }

    pub fn base_size(&self) -> i32 {
        match self {
            Shape::Circle => 1,
            Shape::Square => 10,
            Shape::Triangle => 3,
        }
    }
}
