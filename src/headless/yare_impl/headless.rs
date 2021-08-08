use crate::{
    bindings::{
        base::{
            CIRCLE_START_OFFSET, SPIRIT_COSTS_CIRCLE, SPIRIT_COSTS_SQUARE, SPIRIT_COSTS_TRIANGLE, PRODUCTION_OFFSET, DISRUPTION_RADIUS
        },
        game::MAX_GAME_LEN,
        outpost::NORMAL_RANGE,
        position::Position,
        spirit::{ENERGIZE_RANGE, EXPLODE_DAMAGE, EXPLODE_RADIUS, MOVEMENT_SPEED},
        star::next_energy,
    },
    yare_impl::*,
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

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
    replay: Vec<ReplayTick>
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
            replay: Vec::new()
        }
    }

    fn tick(&mut self) -> Option<Outcome> {
        //dbg!(self.tick);
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

            // sort commands by spirit, and then by command, and dedup to the last command issued
            let mut player_commands: Vec<(usize, Command)> = unsafe { COMMANDS.clone() }
                .into_iter()
                .enumerate()
                .collect();
            //dbg!(player_commands.len());
            player_commands.sort_by(|(a_i, a_command), (b_i, b_command)| {
                // if the commands are for different spirits, sort by spirit index
                if a_command.index() != b_command.index() {
                    return a_command.index().cmp(&b_command.index());
                }
                // if the commands are for the same spirit sort by command id
                if a_command.id() != b_command.id() {
                    return a_command.id().cmp(&b_command.id());
                }
                // if the commands are for the same spirit and are the same command
                // sort the index of the command first
                return b_i.cmp(&a_i);
            });
            // drop all duplicate commands except for the last one submitted for that spirit/command
            player_commands.dedup_by(|(a_i, a_command), (b_i, b_command)| {
                a_command.index() == b_command.index() && a_command.id() == b_command.id()
            });
            //dbg!(player_commands.clone());

            all_commands.push(
                player_commands
                    .into_iter()
                    .map(|(i, command)| command)
                    .collect(),
            );
            unsafe { COMMANDS = Vec::new() }
        }

        
        for player in self.players.iter_mut() {
            if player.base.energy >= player.base.spirit_cost && !player.base.disrupted {
                player.base.energy -= player.base.spirit_cost;
                //dbg!(&player.base);
                let spirit_id = player.spirits.len();
                let offset = &PRODUCTION_OFFSET[player.index];
                let pos = player.base.pos + offset.into();
                player.spirits.push(Spirit::new(player.index, player.shape, pos, spirit_id));
            }
        }

        // TODO do game logic

        let mut charging_spirits = vec![vec![0; 0]; self.stars.len()];
        let mut spirit_energy_changes = vec![0 as i32; spirits.len()];
        let mut base_energy_changes = vec![0; bases.len()];
        let mut outpost_energy_changes =
            vec![vec![0 as i32; self.players.len()]; self.outposts.len()];

        // process energize/explode commands + outposts

        let mut energizes: Vec<ReplayEnergize> = Vec::new();

        for (player_i, player_commands) in all_commands.iter().enumerate() {
            let player = &mut self.players[player_i];

            // TODO: I think we need to track the change in each sprites energy that tick
            // and apply it after all of the commands process
            for command in player_commands.iter() {
                match command {
                    Command::Energize { index, target } => {
                        let source_spirit = &spirits[*index];
                        let target_spirit = &spirits[*target];
                        if source_spirit.hp < 1 || player.index != source_spirit.player_id || source_spirit.pos.dist(target_spirit.pos) > ENERGIZE_RANGE {
                            //dbg!(source_spirit);
                            continue;
                        }
                        // self energize
                        if index == target {
                            for (i, star) in self.stars.iter().enumerate() {
                                // check distance
                                if star.pos.dist(source_spirit.pos) < ENERGIZE_RANGE {
                                    //dbg!(source_spirit);
                                    charging_spirits[i].push(*index);
                                }
                            }
                        } else {
                            // charge friendly
                            energizes.push(ReplayEnergize::spirit_to_spirit(source_spirit, target_spirit));
                            spirit_energy_changes[*index] -= source_spirit.energize_amount();
                            if source_spirit.player_id == target_spirit.player_id {
                                spirit_energy_changes[*target] += source_spirit.energize_amount();
                            } else {
                                // attack
                                spirit_energy_changes[*target] -=
                                    2 * source_spirit.energize_amount();
                            }
                        }
                    }
                    Command::EnergizeBase { index, target } => {
                        let source_spirit = &spirits[*index];
                        let target_base = &bases[*target];
                        if source_spirit.hp < 1 || player.index != source_spirit.player_id || source_spirit.pos.dist(target_base.pos) > ENERGIZE_RANGE {
                            //dbg!(source_spirit);
                            continue;
                        }
                        spirit_energy_changes[*index] -= source_spirit.energize_amount();
                        energizes.push(ReplayEnergize::spirit_to_base(source_spirit, target_base));
                        if source_spirit.player_id == target_base.player_id {
                            // charging base
                            //dbg!(source_spirit, target_base);
                            base_energy_changes[*target] += source_spirit.energize_amount();
                        } else {
                            // attacking enemy
                            //dbg!(source_spirit, target_base);
                            base_energy_changes[*target] -= 2 * source_spirit.energize_amount();
                        }
                    }
                    Command::EnergizeOutpost { index, target } => {
                        let source_spirit = &spirits[*index];
                        let target_outpost = &self.outposts[*target];
                        if source_spirit.hp < 1 || player.index != source_spirit.player_id || source_spirit.pos.dist(target_outpost.pos) > ENERGIZE_RANGE{
                            continue;
                        }
                        
                        energizes.push(ReplayEnergize::spirit_to_outpost(source_spirit, target_outpost));
                        spirit_energy_changes[*index] -= source_spirit.energize_amount();
                        if target_outpost.player_id == source_spirit.player_id
                            || target_outpost.energy == 0
                        {
                            outpost_energy_changes[*target][player.index] +=
                                source_spirit.energize_amount();
                        } else {
                            outpost_energy_changes[*target][player.index] -=
                                2 * source_spirit.energize_amount();
                        }
                    }
                    Command::Explode { index } => {
                        let source_spirit = &spirits[*index];
                        if source_spirit.hp < 1
                            || player.index != source_spirit.player_id
                            || source_spirit.shape != Shape::Triangle
                        {
                            continue;
                        }
                        spirit_energy_changes[*index] = -100000000;
                        for (target, spirit) in spirits.iter().enumerate() {
                            if spirit.hp > 0
                                && spirit.player_id != source_spirit.player_id
                                && spirit.pos.dist(source_spirit.pos) <= ENERGIZE_RANGE
                            {
                                spirit_energy_changes[target] -= EXPLODE_DAMAGE;
                            }
                        }
                    }
                    _ => (),
                }
            }
        }

        // Apply all energy changes to spirits bases and outposts
        for i in 0..spirit_energy_changes.len() {
            let delta = spirit_energy_changes[i];
            let spirit_copy = &spirits[i];
            let spirit = &mut self.players[spirit_copy.player_id].spirits[spirit_copy.id];
            spirit.energy += delta;
        }

        for i in 0..base_energy_changes.len() {
            let delta = base_energy_changes[i];
            //dbg!(delta);
            let base_copy = &bases[i];
            let base = &mut self.players[base_copy.player_id].base;
            base.energy += delta;
            //dbg!(base);
        }

        // process charging from stars
        for i in 0..self.stars.len() {
            let star = &mut self.stars[i];
            if star.active_at <= self.tick {
                star.energy = next_energy(star.energy);
            }
            let indices = &mut charging_spirits[i];
            indices.shuffle(&mut thread_rng());
            for index in indices {
                let spirit_copy = &spirits[*index];
                let spirit = &mut self.players[spirit_copy.player_id].spirits[spirit_copy.id];
                let amount = star.energy.min(spirit.energize_self_amount());
                if amount > 0 {
                    energizes.push(ReplayEnergize::from_star(i, &spirit))
                }
                star.energy -= amount;
                spirit.energy += amount;
            }
            star.energy = star.energy.min(star.energy_cap);
            //dbg!(star);
        }

        // kill sprites with energy < 0

        // move

        for (player_i, player_commands) in all_commands.iter().enumerate() {
            let player = &mut self.players[player_i];

            // TODO: I think we need to track the change in each sprites energy that tick
            // and apply it after all of the commands process
            for command in player_commands.iter() {
                match command {
                    Command::Goto { index, target } => {
                        // TODO: orbit stars/bases/outposts
                        let spirit_copy = &spirits[*index];
                        let spirit =
                            &mut self.players[spirit_copy.player_id].spirits[spirit_copy.id];
                        let dist = spirit.pos.dist(*target).min(MOVEMENT_SPEED);
                        spirit.pos = spirit.pos.towards(*target, dist);
                        if *index == 1 {
                            //dbg!(spirit_copy);
                            //dbg!(spirit);
                        }
                    }
                    _ => (),
                }
            }
        }

        // merge

        // divide

        // jump



        self.replay.push(ReplayTick {
            t: self.tick,
            p1: self.players[0].spirits.iter().filter(|s| s.hp > 0).map(|s| s.into()).collect(),
            p2: self.players[1].spirits.iter().filter(|s| s.hp > 0).map(|s| s.into()).collect(),
            b1: (&self.players[0].base).into(),
            b2: (&self.players[1].base).into(),
            ou: (&self.outposts[0]).into(),
            e: energizes,
            s: Vec::new(),
            g1: Vec::new(),
            g2: Vec::new(),
            st: ReplayStars::new(self.stars[0].energy, self.stars[1].energy, self.stars[2].energy)
        });

        // update bases
        for player in self.players.iter_mut() {
            let mut living_spirits = 0;
            for spirit in player.spirits.iter_mut() {
                if spirit.energy < 0 {
                    spirit.hp = 0
                }
                if spirit.hp > 0 {
                    living_spirits += 1
                }
                spirit.energy = spirit.energy.clamp(0, spirit.energy_cap)
            }
            player.base.spirit_cost = player.shape.spirit_cost(living_spirits);
            if player.base.energy < 0 {
                //dbg!(&player.base);
                if player.base.hp == 1 {
                    let winner = if player.index == 0 { 1 } else { 0 };
                    return Some(Outcome::Victory(winner))
                } else {
                    player.base.hp -= 1
                }
                player.base.energy = 0
            }

            player.base.energy = player.base.energy.clamp(0, player.base.energy_cap);
            //dbg!(player.index, player.spirits.len(), player.base.energy);
        }
        for i in 0..self.players.len() {
            let pos = self.players[i].base.pos;
            let owner = self.players[i].base.player_id;
            let disrupted = self.players.iter().filter(|x| x.index != i && x.spirits.iter().filter(|s| s.hp > 0 && s.pos.dist(pos) <= DISRUPTION_RADIUS).count() > 0).count() > 0;
            self.players[i].base.disrupted = disrupted;
        }


        self.tick += 1;
        if self.tick > MAX_GAME_LEN {
            return Some(Outcome::Draw);
        }

        None
    }

    pub fn simulate(mut self) -> (u32, Outcome) {
        loop {
            if let Some(outcome) = self.tick() {
                let replay = serde_json::to_string(&self.replay).unwrap();
                let mut file = File::create("replay.json").unwrap();
                file.write_all(replay.as_bytes()).unwrap();
                return (self.tick, outcome);
            }
        }
    }
}
