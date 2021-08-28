use super::vec2::Vec2;

#[derive(Clone, Debug, Copy)]
pub(crate) enum Command {
    Goto { index: usize, target: Vec2 },
    Energize { index: usize, target: usize },
    EnergizeBase { index: usize, target: usize },
    EnergizeOutpost { index: usize, target: usize },
    EnergizeStar { index: usize, target: usize },
    Jump { index: usize, target: Vec2 },
    Merge { index: usize, target: usize },
    Divide { index: usize },
    Explode { index: usize },
}

// First, all energize() methods are calculated, then all movements, then all
// merge() and so on. Except for explode() that happens together with
// energize(). energize, explode
// move
// merge
// divide
// jump
impl Command {
    // used to compared whether a command overwrites another one
    pub fn id(&self) -> usize {
        match self {
            Command::Energize { .. } => 0,
            Command::EnergizeBase { .. } => 0,
            Command::EnergizeOutpost { .. } => 0,
            Command::EnergizeStar { .. } => 0,
            Command::Explode { .. } => 1,
            Command::Goto { .. } => 2,
            Command::Merge { .. } => 3,
            Command::Divide { .. } => 4,
            Command::Jump { .. } => 5,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Command::Energize { index, .. } => *index,
            Command::EnergizeBase { index, .. } => *index,
            Command::EnergizeOutpost { index, .. } => *index,
            Command::EnergizeStar { index, .. } => *index,
            Command::Explode { index, .. } => *index,
            Command::Goto { index, .. } => *index,
            Command::Merge { index, .. } => *index,
            Command::Divide { index, .. } => *index,
            Command::Jump { index, .. } => *index,
        }
    }
}
