use std::{
    array, collections::HashMap, fmt::Debug, ops::{Add, Bound, Div, Mul, RangeBounds, Sub},
};

use rand::{
    Rng, distr::{Distribution, Uniform, uniform::SampleUniform},
};
use serde::{Deserialize, Deserializer, Serialize, de::Error, ser::SerializeMap};
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

pub type BaseStats<T> = [T; Stat::COUNT];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Stats<T = u32>
where
    T: Clone + Copy
{
    base_stats: BaseStats<T>,
}

impl<T> Stats<T>
where
    T: Default + Clone + Copy
{
    pub fn new(stats: BaseStats<T>) -> Self { Self { base_stats: stats } }

    pub fn from_iter(stats: impl IntoIterator<Item = (Stat, T)>) -> Self
    {
        let mut result = BaseStats::default();
        for (stat, value) in stats
        {
            result[stat as usize] = value;
        }

        Self { base_stats: result }
    }

    pub fn stat(&self, stat: Stat) -> T { self.base_stats[stat as usize] }

    pub fn set_stat(&mut self, stat: Stat, value: T) { self.base_stats[stat as usize] = value; }

    pub fn deserialize_optional<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>
    {
        let stats = HashMap::<Stat, T>::deserialize(deserializer)?;
        let mut results = BaseStats::default();

        for (stat, value) in stats.into_iter()
        {
            results[stat as usize] = value;
        }

        Ok(Self { base_stats: results })
    }

    pub fn map<F, O>(&self, func: F) -> Stats<O>
    where
        O: Clone + Copy,
        F: Fn(T) -> O,
    {
        Stats { base_stats: self.base_stats.map(func) }
    }
}

impl Serialize for Stats
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
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
        D: serde::Deserializer<'de>,
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

        Ok(Stats { base_stats: stats })
    }
}

macro_rules! impl_math_ops {
    ($($trait:ident, $method:ident),*) => {
        $(
            impl<T> $trait for Stats<T>
            where
                T: Copy + $trait<Output = T>,
            {
                type Output = Self;

                fn $method(self, rhs: Self) -> <Self as $trait>::Output {
                    Self {
                        base_stats: std::array::from_fn(|i| {
                            self.base_stats[i].$method(rhs.base_stats[i])
                        }),
                    }
                }
            }

            impl<T> $trait<T> for Stats<T>
            where
                T: Copy + $trait<Output = T>,
            {
                type Output = Self;

                fn $method(self, scalar: T) -> <Self as $trait>::Output {
                    Self {
                        base_stats: std::array::from_fn(|i| {
                            self.base_stats[i].$method(scalar)
                        }),
                    }
                }
            }
        )*
    };
}

impl_math_ops!(Add, add, Sub, sub, Mul, mul, Div, div);

pub struct StatsDistribution<T = u32>
where
    T: SampleUniform,
{
    sampler: Uniform<T>,
}

impl StatsDistribution<u32>
{
    pub fn new<R: RangeBounds<u32>>(range: R) -> Self
    {
        let start = match range.start_bound()
        {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound()
        {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s - 1,
            Bound::Unbounded => u32::MAX,
        };

        Self {
            sampler: Uniform::new_inclusive(start, end).expect("Bad range. This should be impossible"),
        }
    }
}

impl<T> Distribution<Stats<T>> for StatsDistribution<T>
where
    T: Default + Copy + Clone + SampleUniform,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Stats<T>
    {
        Stats {
            base_stats: array::from_fn(|_| self.sampler.sample(rng)),
        }
    }
}
