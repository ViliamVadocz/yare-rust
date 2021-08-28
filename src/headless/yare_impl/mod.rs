mod base;
mod command;
mod headless;
mod outpost;
mod replaytick;
mod shape;
mod spirit;
mod star;
mod vec2;

pub(crate) use command::Command;
pub use headless::{BotFn, Headless, Outcome, SimulationResult};
pub(crate) use headless::{BASES, COMMANDS, ME, OUTPOSTS, PLAYER_NUM, SPIRITS, STARS};
pub use shape::Shape;
pub(crate) use vec2::Vec2;
