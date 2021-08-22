use std::convert::From;

use serde::{Deserialize, Serialize};

use super::{base::Base, outpost::Outpost, spirit::Spirit, vec2::Vec2};

// x,y
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ReplayPos(f32, f32);
impl From<&Vec2> for ReplayPos {
    fn from(pos: &Vec2) -> ReplayPos {
        ReplayPos(pos.x, pos.y)
    }
}

// name, pos, size, energy, shape
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ReplaySpirit(String, ReplayPos, i32, i32, u32);
impl From<&Spirit> for ReplaySpirit {
    fn from(spirit: &Spirit) -> ReplaySpirit {
        ReplaySpirit(
            spirit_name(spirit),
            (&spirit.pos).into(),
            spirit.size,
            spirit.energy,
            spirit.hp,
        )
    }
}

fn spirit_name(spirit: &Spirit) -> String {
    format!("player{}_{}", spirit.player_id, spirit.id)
}

// source, dst, size
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ReplayEnergize(String, String, i32);

impl ReplayEnergize {
    pub fn from_star(a: usize, b: &Spirit) -> ReplayEnergize {
        let name = match a {
            0 => "star_zxq",
            1 => "star_a1c",
            _ => "star_p89",
        }
        .to_string();
        ReplayEnergize(name, spirit_name(b), b.energize_self_amount())
    }

    pub fn spirit_to_spirit(a: &Spirit, b: &Spirit) -> ReplayEnergize {
        ReplayEnergize(spirit_name(a), spirit_name(b), a.energize_amount())
    }

    pub fn spirit_to_base(a: &Spirit, b: &Base) -> ReplayEnergize {
        ReplayEnergize(
            spirit_name(a),
            format!("base_player{}", b.player_id),
            a.energize_amount(),
        )
    }

    pub fn spirit_to_outpost(a: &Spirit, _b: &Outpost) -> ReplayEnergize {
        ReplayEnergize(
            spirit_name(a),
            "outpost_mdo".to_string(),
            a.energize_amount(),
        )
    }

    pub fn outpost_to_spirit(_b: &Outpost, a: &Spirit) -> ReplayEnergize {
        ReplayEnergize(
            "outpost_mdo".to_string(),
            spirit_name(a),
            a.energize_amount(),
        )
    }
}

// charge, owner ("" if charge == 0)
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ReplayOutpost(i32, String);
impl From<&Outpost> for ReplayOutpost {
    fn from(outpost: &Outpost) -> ReplayOutpost {
        if outpost.energy == 0 {
            ReplayOutpost(0, "".to_string())
        } else {
            ReplayOutpost(outpost.energy, format!("player{}", outpost.player_id))
        }
    }
}

// charge, spirit_cost, disruption, hp
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ReplayBase(i32, i32, i32, u32);
impl From<&Base> for ReplayBase {
    fn from(base: &Base) -> ReplayBase {
        ReplayBase(
            base.energy,
            base.spirit_cost,
            base.disrupted as i32,
            base.hp,
        )
    }
}

// charge zxq, a1c, p89
#[derive(Serialize, Deserialize, Debug)]
pub struct ReplayStars(i32, i32, i32);
impl ReplayStars {
    pub fn new(a: i32, b: i32, c: i32) -> ReplayStars {
        ReplayStars(a, b, c)
    }
}

// "sh", unit, message
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ReplayShout(String, String, String);

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ReplayTick {
    pub t: u32,
    pub p1: Vec<ReplaySpirit>,
    pub p2: Vec<ReplaySpirit>,
    pub b1: ReplayBase,
    pub b2: ReplayBase,
    pub ou: ReplayOutpost,
    pub e: Vec<ReplayEnergize>,
    pub s: Vec<ReplayShout>,
    pub st: ReplayStars,
    pub g1: Vec<String>,
    pub g2: Vec<String>,
}
