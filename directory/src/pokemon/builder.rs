use std::sync::LazyLock;

use rand::{Rng, distr::{Distribution, StandardUniform}};

use crate::pokemon::{BasePokemon, Pokemon, attributes::{gender::{Gender, GenderDistribution}, nature::Nature, stats::{Stats, StatsDistribution}}};

pub enum BuildState<'a, D, T>
where
    D: Distribution<T>,
    T: Default,
{
    Set(T),
    Random(&'a D),
    Default,
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
    experience: u32,
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
    pub fn new(base: &'a BasePokemon, experience: u32) -> Self
    {
        Self {
            base,
            experience,
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
            experience: self.experience,
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
