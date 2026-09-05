use std::{array, iter, sync::LazyLock};

use paste::paste;
use rand::{Rng, RngExt, distr::{Distribution, StandardUniform}};
use serde::{Deserialize, Serialize};
use strum::{EnumCount, FromRepr, VariantArray};

use crate::pokemon::{gender::{Gender, GenderDistribution}, nature::Nature, stats::{BaseStats, Stat, Stats, StatsDistribution}, types::Type};

pub mod types;
pub mod stats;
pub mod nature;
pub mod manager;
pub mod gender;

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
}

#[derive(Debug)]
pub struct Pokemon<'a>
{
    base: &'a BasePokemon,
    level: u32,
    evs: Stats,
    ivs: Stats,
    nature: Nature,
    gender: Gender,
}

impl<'a> Pokemon<'a>
{
    pub fn builder(base: &'a BasePokemon, level: u32) -> PokemonBuilder<'a>
    {
        PokemonBuilder::new(base, level)
    }

    pub fn stat(&self, stat: Stat) -> u32
    {
        let core = <f64>::floor(((2 * self.base.stats.stat(stat) + self.ivs.stat(stat) + (<f64>::floor(self.evs.stat(stat) as f64 / 4.0)) as u32) * self.level) as f64 / 100.0) as u32 + self.level;
        let value = match stat
        {
            Stat::Health => core + 10,
            s => <f64>::floor((core + 5) as f64 * self.nature.get_modifier(s)) as u32
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


pub enum BuildState<'a, D, T>
where
    D: Distribution<T>,
    T: Default,
{
    Set(T),
    Random(&'a D),
    Default
}

impl<'a, D, T> BuildState<'a, D, T>
where
    D: Distribution<T>,
    T: Default,
{
    pub fn get(self, rng: &mut impl Rng) -> T
    {
        match self
        {
            Self::Set(value) => value,
            Self::Random(dist) => dist.sample(rng),
            Self::Default => T::default(),
        }
    }
}

pub struct PokemonBuilder<'a>
{
    base: &'a BasePokemon,
    level: u32,
    evs: BuildState<'a, StatsDistribution, Stats>,
    ivs: BuildState<'a, StatsDistribution, Stats>,
    nature: BuildState<'a, StandardUniform, Nature>,
    gender: BuildState<'a, GenderDistribution, Gender>,
}

static IVS_DIST: LazyLock<StatsDistribution> = LazyLock::new(|| StatsDistribution::new(0..=31));
static EVS_DIST: LazyLock<StatsDistribution> = LazyLock::new(|| StatsDistribution::new(0..=252));
static NATURE_DIST: LazyLock<StandardUniform> = LazyLock::new(|| StandardUniform::default());

macro_rules! impl_builder_methods {
    ($field:ident, $t:ty, $self:tt => $dist:expr) => {
        paste::paste! {
            pub fn [<with_ $field>](mut self, $field: $t) -> Self
            {
                self.$field = BuildState::Set($field);
                self
            }

            pub fn [<with_random_ $field>](mut $self) -> Self
            {
                $self.$field = BuildState::Random($dist);
                $self
            }

            pub fn [<with_default_ $field>](mut self) -> Self
            {
                self.$field = BuildState::Default;
                self
            }
        }
    };
}

impl<'a> PokemonBuilder<'a>
{

    pub fn new(base: &'a BasePokemon, level: u32) -> Self
    {
        Self {
            base,
            level,
            evs: BuildState::Default,
            ivs: BuildState::Random(&*IVS_DIST),
            nature: BuildState::Random(&*NATURE_DIST),
            gender: BuildState::Random(&base.gender_chances),
        }
    }

    pub fn build(self) -> Pokemon<'a>
    {
        let mut rng = rand::rng();

        Pokemon {
            base: self.base,
            level: self.level,
            evs: self.evs.get(&mut rng),
            ivs: self.ivs.get(&mut rng),
            nature: self.nature.get(&mut rng),
            gender: self.gender.get(&mut rng),
        }
    }

    impl_builder_methods!(evs, Stats, self => &*EVS_DIST);
    impl_builder_methods!(ivs, Stats, self => &*IVS_DIST);
    impl_builder_methods!(nature, Nature, self => &*NATURE_DIST);
    impl_builder_methods!(gender, Gender, self => &self.base.gender_chances);
}
