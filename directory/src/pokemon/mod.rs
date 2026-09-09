use std::{collections::HashMap, sync::LazyLock};

use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use strum::{EnumCount, VariantArray};

use crate::{
    moves::{Move, MoveList, manager::MoveNames},
    pokemon::{
        attributes::{
            gender::{Gender, GenderDistribution},
            growth_rate::GrowthRate,
            nature::Nature,
            stats::{Stat, Stats, StatsDistribution},
            types::Type,
        },
        builder::PokemonBuilder,
    },
};

pub mod attributes;
pub mod builder;
pub mod manager;

pub type Learnset = HashMap<u32, SmallVec<[MoveNames; 1]>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BasePokemon
{
    name: String,
    id: String,

    #[serde(deserialize_with = "Type::deserialize_typing")]
    types: (Type, Option<Type>),
    stats: Stats,
    ability: String,

    gender_chances: GenderDistribution,
    #[serde(deserialize_with = "Stats::deserialize_optional")]
    ev_yield: Stats,
    catch_rate: u8,
    base_friendship: u8,
    base_experience: u8,
    growth_rate: GrowthRate,

    learnset: Learnset,
}

#[derive(Debug)]
pub struct Pokemon<'p, 'm>
{
    base: &'p BasePokemon,
    experience: u32,
    evs: Stats,
    ivs: Stats,
    nature: Nature,
    gender: Gender,
    moves: MoveList<'m>,
}

impl<'p, 'm> Pokemon<'p, 'm>
where
    'p: 'm
{
    pub fn builder(base: &'p BasePokemon, experience: u32) -> PokemonBuilder<'p, 'p, 'm>
    {
        PokemonBuilder::new(base, experience)
    }

    pub fn builder_from_level(base: &'p BasePokemon, level: u32) -> PokemonBuilder<'p, 'p, 'm>
    {
        let experience = base.growth_rate.experience_for_level(level);
        PokemonBuilder::new(base, experience)
    }

    pub fn stat(&self, stat: Stat) -> u32
    {
        let level = self.base.growth_rate.level(self.experience);
        let core = <f64>::floor(
            ((2 * self.base.stats.stat(stat)
                + self.ivs.stat(stat)
                + (<f64>::floor(self.evs.stat(stat) as f64 / 4.0)) as u32)
                * level) as f64
                / 100.0,
        ) as u32
            + level;
        let value = match stat
        {
            Stat::Health => core + 10,
            s => <f64>::floor((core + 5) as f64 * self.nature.get_modifier(s)) as u32,
        };

        value
    }

    pub fn stats(&self) -> Stats
    {
        let mut stats = [0_u32; Stat::COUNT];
        for &stat in Stat::VARIANTS
        {
            stats[stat as usize] = self.stat(stat);
        }

        Stats::new(stats)
    }
}
