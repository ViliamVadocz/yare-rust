use crate::yare_impl::Pos;

#[derive(Clone, Debug)]
pub(crate) enum Command {
    Goto { index: usize, target: Pos },
    Energize { index: usize, target: usize },
    EnergizeBase { index: usize, target: usize },
    EnergizeOutpost { index: usize, target: usize },
    Jump { index: usize, target: Pos },
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
    fn priority(&self) -> usize {
        match self {
            Command::Energize { .. } => 0,
            Command::EnergizeBase { .. } => 0,
            Command::EnergizeOutpost { .. } => 0,
            Command::Explode { .. } => 0,
            Command::Goto { .. } => 1,
            Command::Merge { .. } => 2,
            Command::Divide { .. } => 3,
            Command::Jump { .. } => 4,
        }
    }
}
