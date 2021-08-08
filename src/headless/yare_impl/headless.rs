use crate::{
    bindings::{
        base::{
            CIRCLE_START_OFFSET,
            SPIRIT_COSTS_CIRCLE,
            SPIRIT_COSTS_SQUARE,
            SPIRIT_COSTS_TRIANGLE,
        },
        game::MAX_GAME_LEN,
        outpost::NORMAL_RANGE,
        position::Position,
        star::next_energy,
    },
    yare_impl::*,
};

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
    base: Base,
    spirits: Vec<Spirit>,
}

pub struct Headless<F: Fn(u32)> {
    players: Vec<Player<F>>,
    stars: Vec<Star>,
    outposts: Vec<Outpost>,
    tick: u32,
}

impl<F: Fn(u32)> Headless<F> {
    pub fn init(bots: Vec<F>, shapes: Vec<Shape>) -> Self {
        let players = bots
            .into_iter()
            .zip(shapes.into_iter())
            .enumerate()
            .map(|(index, (func, shape))| Player {
                index,
                func,
                shape,
                base: Base::game_start(index, &shape),
                spirits: Spirit::game_start(index, &shape),
            })
            .collect();
        Self {
            players,
            stars: Star::game_start(),
            outposts: Outpost::game_start(),
            tick: 0,
        }
    }

    fn tick(&mut self) -> Option<Outcome> {
        let mut spirits: Vec<Spirit> = self
            .players
            .iter()
            .map(|player| player.spirits.clone())
            .flatten()
            .collect();
        unsafe { SPIRITS = spirits.clone() };

        let mut bases: Vec<Base> = self
            .players
            .iter()
            .map(|player| player.base.clone())
            .collect();
        unsafe { BASES = bases.clone() };
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
            star.energy = next_energy(star.energy);
        }

        let mut spirit_energy_changes = vec![0 as i32; spirits.len()];

        // process energize/explode commands + outposts
        for (player_i, player_commands) in all_commands.iter().enumerate() {
            let player = &mut self.players[player_i];

            // TODO: I think we need to track the change in each sprites energy that tick
            // and apply it after all of the commands process
            for command in player_commands.iter() {
                match command {
                    Command::Energize { index, target } => {
                        let source_spirit = &spirits[*index];
                        let target_spirit = &spirits[*target];
                        if !source_spirit.hp > 0 || player.index != source_spirit.player_id {
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
                    }
                    Command::EnergizeBase { index, target } => {}
                    Command::EnergizeOutpost { index, target } => {}
                    Command::Explode { index } => {}
                    _ => (),
                }
            }
        }

        // TODO: Reconcile the energy differences, kill sprites with energy < 0? or do
        // sprites that were attacked and went down to 0 also die

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
