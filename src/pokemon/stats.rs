use std::{array, collections::HashMap, ops::{Bound, RangeBounds}};

use rand::{Rng, distr::{Distribution, Uniform}};
use serde::{de::Error, Deserialize, Serialize, ser::SerializeMap};
use strum::{EnumCount, VariantArray};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, EnumCount, VariantArray)]
#[serde(rename_all = "snake_case")]
pub enum Stat
{
    Health,
    Attack,
    Defence,
    SpecialAttack,
    SpecialDefence,
    Speed,
}

pub type BaseStats = [u32; Stat::COUNT];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Stats
{
    base_stats: BaseStats,
}

impl Stats
{
    pub fn new(stats: BaseStats) -> Self
    {
        Self {
            base_stats: stats
        }
    }

    pub fn from_iter(stats: impl IntoIterator<Item = (Stat, u32)>) -> Self
    {
        let mut result = BaseStats::default();
        for (stat, value) in stats
        {
            result[stat as usize] = value;
        }

        Self {
            base_stats: result
        }
    }

    pub fn stat(&self, stat: Stat) -> u32
    {
        self.base_stats[stat as usize]
    }
}

impl Serialize for Stats
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        let mut map = serializer.serialize_map(Some(Stat::VARIANTS.len()))?;
        for &stat in Stat::VARIANTS
        {
            map.serialize_entry(&stat, &self.base_stats[stat as usize])?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Stats
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let raw_map = HashMap::<Stat, u32>::deserialize(deserializer)?;
        let mut stats = [0_u32; Stat::COUNT];

        for &stat in Stat::VARIANTS
        {
            if let Some(&val) = raw_map.get(&stat)
            {
                stats[stat as usize] = val;
            }
            else
            {
                return Err(D::Error::custom(format!("Missing stat field: {:?}", stat)));
            }
        }

        Ok(Stats { base_stats: stats } )
    }
}

pub struct StatsDistribution
{
    sampler: Uniform<u32>
}

impl StatsDistribution
{
    pub fn new<R: RangeBounds<u32>>(range: R) -> Self
    {
        let start = match range.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s - 1,
            Bound::Unbounded => u32::MAX
        };

        Self {
            sampler: Uniform::new_inclusive(start, end).expect("Bad range. This should be impossible")
        }
    }
}

impl Distribution<Stats> for StatsDistribution
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Stats
    {
        Stats {
            base_stats: array::from_fn(|_| self.sampler.sample(rng))
        }
    }
}
