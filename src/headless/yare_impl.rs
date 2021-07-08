pub(crate) struct Pos {
    pub x: f32,
    pub y: f32,
}

pub(crate) enum Command {
    Goto { index: usize, target: Pos },
    Energize { index: usize, target: usize },
    EnergizeBase { index: usize, target: usize },
    EnergizeOutpost { index: usize, target: usize },
    Jump { index: usize, target: Pos },
    Merge { index: usize, target: usize },
    Divide { index: usize }
}

#[derive(Clone, Copy)]
pub(crate) enum Shape {
    Circle,
    Square,
    Triangle,
}

impl Into<usize> for Shape {
    fn into(self) -> usize {
        match self {
            Shape::Circle => 0,
            Shape::Square => 1,
            Shape::Triangle => 2,
        }
    }
}

pub(crate) struct Spirit {
    pub energy_cap: u32,
    pub energy: u32,
    pub hp: u32,
    pub id: usize,
    pub player_id: usize,
    pub pos: Pos,
    pub shape: Shape,
    pub size: u32,
}
pub(crate) struct Base {
    pub energy_cap: u32,
    pub energy: u32,
    pub hp: u32,
    pub player_id: usize,
    pub pos: Pos,
    pub spirit_cost: u32,
}
pub(crate) struct Star {
    pub energy_cap: u32,
    pub energy: u32,
    pub pos: Pos,
}
pub(crate) struct Outpost {
    pub energy_cap: u32,
    pub energy: u32,
    pub player_id: usize,
    pub pos: Pos,
    pub range: f32,
}

pub enum Outcome {
    Victory(usize),
    Draw,
}

pub(crate) static mut COMMANDS: Vec<Command> = Vec::new();
pub(crate) static mut SPIRITS: Vec<Spirit> = Vec::new();
pub(crate) static mut BASES: Vec<Base> = Vec::new();
pub(crate) static mut STARS: Vec<Star> = Vec::new();
pub(crate) static mut OUTPOSTS: Vec<Outpost> = Vec::new();
pub(crate) static mut ME: usize = 0;
pub(crate) static mut PLAYER_NUM: usize = 2;

pub struct Headless<F: Fn(u32)> {
    bots: Vec<F>,
}

impl<F: Fn(u32)> Headless<F> {
    pub fn init(bots: Vec<F>) -> Self {
        // TODO Create all objects.
        Self {
            bots
        }
    }

    pub fn simulate(self) -> Outcome {
        Outcome::Draw
    }
}
