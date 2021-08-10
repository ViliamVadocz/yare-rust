use std::{fs::File, io::prelude::*, mem, rc::Rc};

use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::{
    bindings::{
        base::{
            CIRCLE_START_OFFSET,
            DISRUPTION_RADIUS,
            PRODUCTION_OFFSET,
            SPIRIT_COSTS_CIRCLE,
            SPIRIT_COSTS_SQUARE,
            SPIRIT_COSTS_TRIANGLE,
        },
        game::MAX_GAME_LEN,
        outpost::NORMAL_RANGE,
        position::Position,
        spirit::{
            ENERGIZE_RANGE,
            EXPLODE_DAMAGE,
            EXPLODE_RADIUS,
            JUMP_COST_PER_DIST,
            MAX_CIRCLE_SIZE,
            MERGE_DISTANCE,
            MOVEMENT_SPEED,
        },
        star::next_energy,
    },
    yare_impl::*,
};

#[repr(C)]
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

pub type BotFn = Fn(u32);

struct Player {
    index: usize,
    func: Rc<BotFn>,
    shape: Shape,
    base: Base,
    spirits: Vec<Spirit>,
}

pub struct Headless {
    players: Vec<Player>,
    stars: Vec<Star>,
    outposts: Vec<Outpost>,
    tick: u32,
    replay: Vec<ReplayTick>,

    all_commands: Vec<Vec<Command>>,
    charging_spirits: Vec<Vec<usize>>,
    spirit_energy_changes: Vec<i32>,
    base_energy_changes: Vec<i32>,
    outpost_energy_changes: Vec<Vec<i32>>,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SimulationResult(u32, Outcome);

impl Headless {
    pub fn init(bots: &[Rc<BotFn>], shapes: &[Shape]) -> Self {
        let players: Vec<Player> = bots
            .into_iter()
            .zip(shapes.into_iter())
            .enumerate()
            .map(|(index, (func, &shape))| Player {
                index,
                func: func.clone(),
                shape,
                base: Base::game_start(index, &shape),
                spirits: Spirit::game_start(index, &shape),
            })
            .collect();
        let stars = Star::game_start();
        let outposts = Outpost::game_start();
        let player_len = players.len();
        let stars_len = stars.len();
        let outposts_len = outposts.len();
        Self {
            players,
            stars,
            outposts,
            tick: 0,
            replay: Vec::new(),
            all_commands: vec![Vec::new(); player_len],
            charging_spirits: vec![vec![0; 0]; stars_len],
            spirit_energy_changes: Vec::with_capacity(200),
            base_energy_changes: vec![0; player_len],
            outpost_energy_changes: vec![vec![0 as i32; player_len]; outposts_len],
        }
    }

    pub fn update_env(&mut self) {
        let mut spirits: Vec<Spirit> = self
            .players
            .iter()
            .flat_map(|player| player.spirits.clone())
            .collect();
        unsafe { SPIRITS = spirits };

        let mut bases: Vec<Base> = self
            .players
            .iter()
            .map(|player| player.base.clone())
            .collect();
        unsafe { BASES = bases };
        unsafe { STARS = self.stars.clone() };
        unsafe { OUTPOSTS = self.outposts.clone() };
    }

    pub fn gather_commands(&mut self, player_index: usize) {
        let player = &self.players[player_index];
        unsafe { ME = player.index };
        (player.func)(self.tick);

        // sort commands by spirit, and then by command, and dedup to the last command
        // issued
        let mut player_commands: Vec<(usize, &Command)> =
            unsafe { &COMMANDS }.into_iter().enumerate().collect();
        // dbg!(player_commands.len());
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
        // drop all duplicate commands except for the last one submitted for that
        // spirit/command
        player_commands.dedup_by(|(a_i, a_command), (b_i, b_command)| {
            a_command.index() == b_command.index() && a_command.id() == b_command.id()
        });

        self.all_commands[player.index].clear();
        for command in player_commands {
            self.all_commands[player.index].push(*command.1);
        }
        // dbg!(player_commands.clone());

        unsafe { COMMANDS = Vec::new() }
    }

    pub fn process_commands(&mut self) -> Option<Outcome> {
        let spirits = unsafe { &SPIRITS };
        let bases = unsafe { &BASES };

        for player in self.players.iter_mut() {
            if player.base.energy >= player.base.spirit_cost && !player.base.disrupted {
                player.base.energy -= player.base.spirit_cost;
                // dbg!(&player.base);
                let spirit_id = player.spirits.len();
                let offset = &PRODUCTION_OFFSET[player.index];
                let pos = player.base.pos + offset.into();
                player
                    .spirits
                    .push(Spirit::new(player.index, player.shape, pos, spirit_id));
            }
        }

        for x in self.charging_spirits.iter_mut() {
            x.clear();
        }
        self.spirit_energy_changes.clear();
        self.spirit_energy_changes.resize(spirits.len(), 0);
        self.base_energy_changes.clear();
        self.base_energy_changes.resize(self.players.len(), 0);
        for x in self.outpost_energy_changes.iter_mut() {
            x.clear();
            x.resize(self.players.len(), 0);
        }

        // process energize/explode commands + outposts

        let mut energizes: Vec<ReplayEnergize> = Vec::new();

        for (player_i, player_commands) in self.all_commands.iter().enumerate() {
            let player = &mut self.players[player_i];

            for command in player_commands.iter() {
                match command {
                    Command::Energize { index, target } => {
                        let source_spirit = &spirits[*index];
                        let target_spirit = &spirits[*target];
                        if source_spirit.hp < 1
                            || player.index != source_spirit.player_id
                            || source_spirit.pos.dist(target_spirit.pos) > ENERGIZE_RANGE
                        {
                            // dbg!(source_spirit);
                            continue;
                        }
                        // self energize
                        if index == target {
                            for (i, star) in self.stars.iter().enumerate() {
                                // check distance
                                if star.pos.dist(source_spirit.pos) <= ENERGIZE_RANGE {
                                    // dbg!(source_spirit);
                                    self.charging_spirits[i].push(*index);
                                }
                            }
                        } else {
                            // charge friendly
                            energizes.push(ReplayEnergize::spirit_to_spirit(
                                source_spirit,
                                target_spirit,
                            ));
                            self.spirit_energy_changes[*index] -= source_spirit.energize_amount();
                            if source_spirit.player_id == target_spirit.player_id {
                                self.spirit_energy_changes[*target] +=
                                    source_spirit.energize_amount();
                            } else {
                                // attack
                                self.spirit_energy_changes[*target] -=
                                    2 * source_spirit.energize_amount();
                            }
                        }
                    }
                    Command::EnergizeBase { index, target } => {
                        let source_spirit = &spirits[*index];
                        let target_base = &bases[*target];
                        if source_spirit.hp < 1
                            || player.index != source_spirit.player_id
                            || source_spirit.pos.dist(target_base.pos) > ENERGIZE_RANGE
                        {
                            // dbg!(source_spirit);
                            continue;
                        }
                        self.spirit_energy_changes[*index] -= source_spirit.energize_amount();
                        energizes.push(ReplayEnergize::spirit_to_base(source_spirit, target_base));
                        if source_spirit.player_id == target_base.player_id {
                            // charging base
                            // dbg!(source_spirit, target_base);
                            self.base_energy_changes[*target] += source_spirit.energize_amount();
                        } else {
                            // attacking enemy
                            // dbg!(source_spirit, target_base);
                            self.base_energy_changes[*target] -=
                                2 * source_spirit.energize_amount();
                        }
                    }
                    Command::EnergizeOutpost { index, target } => {
                        let source_spirit = &spirits[*index];
                        let target_outpost = &self.outposts[*target];
                        if source_spirit.hp < 1
                            || player.index != source_spirit.player_id
                            || source_spirit.pos.dist(target_outpost.pos) > ENERGIZE_RANGE
                        {
                            continue;
                        }

                        energizes.push(ReplayEnergize::spirit_to_outpost(
                            source_spirit,
                            target_outpost,
                        ));
                        self.spirit_energy_changes[*index] -= source_spirit.energize_amount();
                        if target_outpost.player_id == source_spirit.player_id
                            || target_outpost.energy == 0
                        {
                            self.outpost_energy_changes[*target][player.index] +=
                                source_spirit.energize_amount();
                        } else {
                            self.outpost_energy_changes[*target][player.index] -=
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
                        self.spirit_energy_changes[*index] = -100000000;
                        for (target, spirit) in spirits.iter().enumerate() {
                            if spirit.hp > 0
                                && spirit.player_id != source_spirit.player_id
                                && spirit.pos.dist(source_spirit.pos) <= ENERGIZE_RANGE
                            {
                                self.spirit_energy_changes[target] -= EXPLODE_DAMAGE;
                            }
                        }
                    }
                    _ => (),
                }
            }
        }

        for i in 0..self.outposts.len() {
            let outpost = &mut self.outposts[i];
            if outpost.energy > 0 {
                // attack a single random nearby spirit
                let mut nearby_spirits: Vec<(usize, &Spirit)> = spirits
                    .iter()
                    .enumerate()
                    .filter(|(i, s)| {
                        s.hp > 0
                            && s.player_id != outpost.player_id
                            && s.pos.dist(outpost.pos) <= outpost.get_range()
                    })
                    .collect();
                if nearby_spirits.len() > 0 {
                    nearby_spirits.shuffle(&mut thread_rng());
                    let target = nearby_spirits[0].0;
                    let attack = outpost.get_attack_energy();
                    outpost.energy -= attack;
                    self.spirit_energy_changes[target] -= 2 * attack;

                    energizes.push(ReplayEnergize::outpost_to_spirit(
                        outpost,
                        &nearby_spirits[0].1,
                    ))
                }
            }
            let deltas = &self.outpost_energy_changes[i];
            if outpost.energy == 0 {
                // players fight for control
                let mut largest_i = 0;
                let mut largest = 0;
                let mut other_sum = 0;
                for (i, &delta) in deltas.iter().enumerate() {
                    if delta > largest {
                        other_sum += largest;
                        largest = delta;
                        largest_i = i;
                    } else {
                        other_sum += delta;
                    }
                }
                if largest > other_sum {
                    outpost.energy = largest - other_sum;
                    outpost.player_id = largest_i;
                }
            } else {
                // players attack
                for delta in deltas {
                    outpost.energy += delta;
                }
            }
            outpost.energy = outpost.energy.clamp(0, outpost.energy_cap);
        }

        // Apply all energy changes to spirits bases and outposts
        for i in 0..self.spirit_energy_changes.len() {
            let delta = self.spirit_energy_changes[i];
            let spirit_copy = &spirits[i];
            let spirit = &mut self.players[spirit_copy.player_id].spirits[spirit_copy.id];
            spirit.energy += delta;
        }

        for i in 0..self.base_energy_changes.len() {
            let delta = self.base_energy_changes[i];
            let base_copy = &bases[i];
            let base = &mut self.players[base_copy.player_id].base;
            base.energy += delta;
        }

        // process charging from stars
        for i in 0..self.stars.len() {
            let star = &mut self.stars[i];
            if star.active_at <= self.tick {
                star.energy = next_energy(star.energy);
            }
            let indices = &mut self.charging_spirits[i];
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
        }

        // move

        for (player_i, player_commands) in self.all_commands.iter().enumerate() {
            let player = &mut self.players[player_i];

            for command in player_commands.iter() {
                match command {
                    Command::Goto { index, target } => {
                        // TODO: orbit stars/bases/outposts
                        let spirit_copy = &spirits[*index];
                        let spirit =
                            &mut self.players[spirit_copy.player_id].spirits[spirit_copy.id];
                        let dist = spirit.pos.dist(*target).min(MOVEMENT_SPEED);
                        spirit.pos = spirit.pos.towards(*target, dist);
                    }
                    _ => (),
                }
            }
        }

        // merge
        for (player_i, player_commands) in self.all_commands.iter().enumerate() {
            let player = &mut self.players[player_i];

            for command in player_commands.iter() {
                match command {
                    Command::Merge { index, target } => {
                        // TODO: orbit stars/bases/outposts
                        let dead_spirit_copy = &spirits[*index];
                        let dead_spirit_copy = self.players[dead_spirit_copy.player_id].spirits
                            [dead_spirit_copy.id]
                            .clone();

                        let merged_spirit_copy = &spirits[*target];
                        let merged_spirit_copy = self.players[merged_spirit_copy.player_id].spirits
                            [merged_spirit_copy.id]
                            .clone();

                        if dead_spirit_copy.hp < 1
                            || merged_spirit_copy.hp < 1
                            || merged_spirit_copy.energy < 0
                            || dead_spirit_copy.energy < 0
                            || player_i != dead_spirit_copy.player_id
                            || merged_spirit_copy.player_id != dead_spirit_copy.player_id
                            || merged_spirit_copy.shape != Shape::Circle
                            || dead_spirit_copy.shape != Shape::Circle
                            || dead_spirit_copy.pos.dist(merged_spirit_copy.pos) > MERGE_DISTANCE
                            || dead_spirit_copy.size + merged_spirit_copy.size > MAX_CIRCLE_SIZE
                        {
                            // dbg!(source_spirit);
                            continue;
                        }
                        self.players[dead_spirit_copy.player_id].spirits[dead_spirit_copy.id].hp =
                            0;
                        self.players[dead_spirit_copy.player_id].spirits[dead_spirit_copy.id]
                            .size = 1;
                        self.players[dead_spirit_copy.player_id].spirits[dead_spirit_copy.id]
                            .energy = 0;
                        self.players[dead_spirit_copy.player_id].spirits[dead_spirit_copy.id]
                            .energy_cap = 10;

                        self.players[merged_spirit_copy.player_id].spirits
                            [merged_spirit_copy.id]
                            .energy += dead_spirit_copy.energy;
                        self.players[merged_spirit_copy.player_id].spirits
                            [merged_spirit_copy.id]
                            .size += dead_spirit_copy.size;
                        self.players[merged_spirit_copy.player_id].spirits[merged_spirit_copy.id]
                            .energy_cap = self.players[merged_spirit_copy.player_id].spirits
                            [merged_spirit_copy.id]
                            .size
                            * 10;
                    }
                    _ => (),
                }
            }
        }

        // divide

        // jump
        for (player_i, player_commands) in self.all_commands.iter().enumerate() {
            let player = &mut self.players[player_i];

            for command in player_commands.iter() {
                match command {
                    Command::Jump { index, target } => {
                        // TODO: exclude star/base positions from jump
                        let spirit_copy = &spirits[*index];
                        let spirit =
                            &mut self.players[spirit_copy.player_id].spirits[spirit_copy.id];
                        let dist = spirit
                            .pos
                            .dist(*target)
                            .min(spirit.energy as f32 / JUMP_COST_PER_DIST);
                        let cost = (dist * JUMP_COST_PER_DIST).ceil() as i32;
                        spirit.energy -= cost;
                        spirit.pos = spirit.pos.towards(*target, dist);
                    }
                    _ => (),
                }
            }
        }

        // update bases
        // kill sprites with energy < 0
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
                if player.base.hp == 1 {
                    let winner = if player.index == 0 { 1 } else { 0 };
                    return Some(Outcome::Victory(winner));
                } else {
                    player.base.hp -= 1
                }
                player.base.energy = 0
            }

            player.base.energy = player.base.energy.clamp(0, player.base.energy_cap);
        }
        for i in 0..self.players.len() {
            let pos = self.players[i].base.pos;
            let owner = self.players[i].base.player_id;
            let disrupted = self
                .players
                .iter()
                .filter(|x| {
                    x.index != i
                        && x.spirits
                            .iter()
                            .filter(|s| s.hp > 0 && s.pos.dist(pos) <= DISRUPTION_RADIUS)
                            .count()
                            > 0
                })
                .count()
                > 0;
            self.players[i].base.disrupted = disrupted;
        }

        self.replay.push(ReplayTick {
            t: self.tick,
            p1: self.players[0].spirits.iter().map(|s| s.into()).collect(),
            p2: self.players[1].spirits.iter().map(|s| s.into()).collect(),
            b1: (&self.players[0].base).into(),
            b2: (&self.players[1].base).into(),
            ou: (&self.outposts[0]).into(),
            e: energizes,
            s: Vec::new(),
            g1: Vec::new(),
            g2: Vec::new(),
            st: ReplayStars::new(
                self.stars[0].energy,
                self.stars[1].energy,
                self.stars[2].energy,
            ),
        });

        self.tick += 1;
        if self.tick > MAX_GAME_LEN {
            return Some(Outcome::Draw);
        }

        None
    }

    pub fn tick(&mut self) -> Option<Outcome> {
        self.update_env();

        for player_index in 0..self.players.len() {
            self.gather_commands(player_index)
        }
        self.process_commands()
    }

    pub fn simulate(mut self) -> SimulationResult {
        loop {
            if let Some(outcome) = self.tick() {
                let replay = serde_json::to_string(&self.replay).unwrap();
                let mut file = File::create("replay.json").unwrap();
                file.write_all(replay.as_bytes()).unwrap();
                return SimulationResult(self.tick, outcome);
            }
        }
    }
}

// expose ability to simulate headless via FFI
// beginnings of allowing training of a bot from python
type TickFn = unsafe extern "C" fn(u32);

// (tick, -1 for still going. 0 or 1 == winner. 2 == draw)
#[repr(C)]
pub struct ExternResult(u32, i32);

impl From<SimulationResult> for ExternResult {
    fn from(result: SimulationResult) -> ExternResult {
        let SimulationResult(tick, outcome) = result;
        let res = match outcome {
            Outcome::Victory(x) => x,
            Outcome::Draw => 2,
        };
        ExternResult(tick, res as i32)
    }
}

pub unsafe extern "C" fn headless_simulate(
    f1: TickFn,
    s1: usize,
    f2: TickFn,
    s2: usize,
) -> ExternResult {
    let bot1: Rc<BotFn> = Rc::new(move |x| {
        f1(x);
    });
    let bot2: Rc<BotFn> = Rc::new(move |x| {
        f2(x);
    });
    let bots = [bot1, bot2];
    let headless = Headless::init(&bots, &[s1.into(), s2.into()]);
    let result = headless.simulate();
    result.into()
}

pub unsafe extern "C" fn headless_init(
    f1: TickFn,
    s1: usize,
    f2: TickFn,
    s2: usize,
) -> *mut Headless {
    let bot1: Rc<BotFn> = Rc::new(move |x| {
        f1(x);
    });
    let bot2: Rc<BotFn> = Rc::new(move |x| {
        f2(x);
    });
    let bots = [bot1, bot2];
    Box::into_raw(Box::new(Headless::init(&bots, &[s1.into(), s2.into()])))
}

pub unsafe extern "C" fn headless_update_env(ptr: *mut Headless) {
    let mut headless = Box::from_raw(ptr);
    headless.update_env();
    mem::forget(headless);
}

pub unsafe extern "C" fn headless_gather_commands(ptr: *mut Headless, player_index: usize) {
    let mut headless = Box::from_raw(ptr);
    headless.gather_commands(player_index);
    mem::forget(headless);
}

pub unsafe extern "C" fn headless_process_commands(ptr: *mut Headless) -> ExternResult {
    let mut headless = Box::from_raw(ptr);

    let tick = headless.tick;
    let res = headless.process_commands();
    mem::forget(headless);
    match res {
        Some(outcome) => match outcome {
            Outcome::Victory(i) => ExternResult(tick, i as i32),
            _ => ExternResult(tick, 2),
        },
        _ => ExternResult(tick, -1),
    }
}
pub unsafe extern "C" fn headless_free(ptr: *mut Headless) {
    let _headless = Box::from_raw(ptr);
}
