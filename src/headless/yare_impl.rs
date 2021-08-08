use crate::bindings::{
    base::{CIRCLE_START_OFFSET, SPIRIT_COSTS_CIRCLE, SPIRIT_COSTS_SQUARE, SPIRIT_COSTS_TRIANGLE},
    game::MAX_GAME_LEN,
    outpost::NORMAL_RANGE,
    position::Position,
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Pos {
    pub x: f32,
    pub y: f32,
}

impl From<&Position> for Pos {
    fn from(pos: &Position) -> Pos {
        Pos { x: pos.x, y: pos.y }
    }
}

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

/*
First, all energize() methods are calculated, then all movements, then all merge() and so on. Except for explode() that happens together with energize().
energize, explode
move
merge
divide
jump
*/
impl Command {
    fn priority(&self) -> usize {
        match self {
            Command::Energize{..} => 0,
            Command::EnergizeBase{..} => 0,
            Command::EnergizeOutpost{..} => 0,
            Command::Explode{..} => 0,
            Command::Goto{..} => 1,
            Command::Merge{..} => 2,
            Command::Divide{..} => 3,
            Command::Jump{..} => 4,
        }
    }
}

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

#[derive(Clone, Debug)]
pub(crate) struct Spirit {
    pub energy_cap: i32,
    pub energy: i32,
    pub hp: u32,
    pub id: usize,
    pub player_id: usize,
    pub pos: Pos,
    pub shape: Shape,
    pub size: i32,
}

impl Spirit {
    fn new(player_id: usize, shape: Shape, pos: Pos, id: usize) -> Spirit {
        let size = match &shape {
            Shape::Circle => 1,
            Shape::Square => 10,
            Shape::Triangle => 3,
        };
        Spirit {
            energy_cap: size * 10,
            energy: size * 10,
            hp: 1,
            id: id, // TODO get id of last created spirit
            player_id,
            pos,
            shape,
            size,
        }
    }

    fn game_start(player_id: usize, shape: &Shape, &mut spirits: &mut Vec<Spirit>) {
        // TODO Create spirits in starting positions.
        match shape {
            Shape::Circle => { 
                for p in CIRCLE_START_OFFSET[player_id]
                .iter() {
                    let id = spirits.len();
                    spirits.push(Spirit::new(player_id, *shape, p.into(), id))
                }
            },
            Shape::Square => vec![],
            Shape::Triangle => vec![],
        };
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Base {
    pub energy_cap: i32,
    pub energy: i32,
    pub hp: u32,
    pub player_id: usize,
    pub pos: Pos,
    pub spirit_cost: i32,
}

impl Base {
    fn game_start(player_id: usize, shape: &Shape) -> Base {
        Base {
            energy_cap: match shape {
                Shape::Circle => 400,
                Shape::Square => 1000,
                Shape::Triangle => 600,
            },
            energy: 0,
            hp: 1,
            player_id,
            pos: if player_id == 0 {
                Pos { x: 1600., y: 700. }
            } else {
                Pos { x: 2600., y: 1700. }
            },
            spirit_cost: match shape {
                Shape::Circle => SPIRIT_COSTS_CIRCLE[0].1,
                Shape::Square => SPIRIT_COSTS_SQUARE[0].1,
                Shape::Triangle => SPIRIT_COSTS_TRIANGLE[0].1,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Star {
    pub energy_cap: i32,
    pub energy: i32,
    pub pos: Pos,
    pub active_at: u32,
}

impl Star {
    fn game_start() -> Vec<Star> {
        vec![
            Star {
                energy_cap: 1000,
                energy: 0,
                pos: Pos { x: 1000., y: 1000. },
                active_at: 0,
            },
            Star {
                energy_cap: 1000,
                energy: 0,
                pos: Pos { x: 2000., y: 1300. },
                active_at: 100,
            },
            Star {
                energy_cap: 1000,
                energy: 0,
                pos: Pos { x: 3200., y: 1400. },
                active_at: 0,
            },
        ]
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Outpost {
    pub energy_cap: i32,
    pub energy: i32,
    pub player_id: usize,
    pub pos: Pos,
    pub range: f32,
}

impl Outpost {
    fn game_start() -> Vec<Outpost> {
        vec![Outpost {
            energy_cap: 1000,
            energy: 0,
            player_id: usize::MAX,
            pos: Pos { x: 2200., y: 1100. },
            range: NORMAL_RANGE,
        }]
    }
}

#[derive(Clone, Debug)]
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

struct Player<F: Fn(u32)> {
    index: usize,
    func: F,
    shape: Shape,
}

pub struct Headless<F: Fn(u32)> {
    players: Vec<Player<F>>,
    stars: Vec<Star>,
    outposts: Vec<Outpost>,
    spirits: Vec<Spirit>,
    bases: Vec<Base>,
    tick: u32,
}

impl<F: Fn(u32)> Headless<F> {
    pub fn init(bots: Vec<F>, shapes: Vec<Shape>) -> Self {
        let mut players = Vec::new();
        let mut spirits = Vec::new();
        let mut bases = Vec::new();

        for (index, (func, shape)) in bots
            .into_iter()
            .zip(shapes.into_iter())
            .enumerate() {
                players.push(Player {
                    index,
                    func,
                    shape,
                });
                bases.push(Base::game_start(index, &shape));
                Spirit::game_start(index, &shape, &mut spirits);
        }
        Self {
            players,
            spirits,
            bases,
            stars: Star::game_start(),
            outposts: Outpost::game_start(),
            tick: 0,
        }
    }

    fn tick(&mut self) -> Option<Outcome> {
        unsafe { SPIRITS = self.spirits.clone() };
        unsafe { BASES = self.bases.clone(); };
        unsafe { STARS = self.stars.clone() };
        unsafe { OUTPOSTS = self.outposts.clone() };

        let mut all_commands: Vec<Vec<Command>> = Vec::with_capacity(self.players.len());
        for player in &self.players {
            unsafe { ME = player.index };
            (player.func)(self.tick);
            all_commands.push(unsafe { COMMANDS.clone() });
            unsafe { COMMANDS = Vec::new() }
        }

        // TODO do game logic

        let mut charging_spirits = Vec::new();
        // first update stars
        for star in &mut self.stars {
            charging_spirits.push(Vec::<usize>::new());
            if star.energy < star.energy_cap {
                let gain = (star.energy as f32 / 100.).round();
                let new_energy = star.energy + 3 + gain as i32;
                star.energy = new_energy.min(star.energy_cap);
            }
        }

        // process energize/explode commands + outposts
        for (player_i, player_commands) in all_commands.iter().enumerate() {
            let player = &mut self.players[player_i];

            // TODO: I think we need to track the change in each sprites energy that tick
            // and apply it after all of the commands process
            for command in player_commands.iter() {
                match command {
                    Command::Energize{index, target} => {
                        let source_spirit = self.spirits[*index];
                        let target_spirit = self.spirits[*target];
                        if !source_spirit.hp > 0 {
                            continue;
                        }
                        // self energize
                        if index == target {
                            for star in &self.stars {
                                // check distance
                            }
                        } else {
                            // charge friendly
                            if source_spirit.player_id == target_spirit.player_id {

                            // attack
                            } else {

                            }
                        }
                    },
                    Command::EnergizeBase{index, target} => {

                    },
                    Command::EnergizeOutpost{index, target} => {

                    },
                    Command::Explode{index} => {

                    },
                    _ => ()
                }
            }
        }

        // TODO: Reconcile the energy differences, kill sprites with energy < 0? or do sprites that were attacked and went down to 0 also die


        // move

        // merge

        // divide

        // jump


        self.tick += 1;
        if self.tick > MAX_GAME_LEN {
            return Some(Outcome::Draw);
        }

        None
    }

    pub fn simulate(mut self) -> Outcome {
        loop {
            if let Some(outcome) = self.tick() {
                return outcome;
            }
        }
    }
}
