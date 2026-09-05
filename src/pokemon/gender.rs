use color_eyre::eyre;
use rand::{Rng, distr::{Distribution, weighted::WeightedIndex}};
use serde::{Deserialize, Serialize};
use strum::{EnumCount, FromRepr};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, EnumCount, FromRepr)]
pub enum Gender { Female, Male, #[default] None }
pub type GenderChances = [f64; Gender::COUNT];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "GenderChances", into = "GenderChances")]
pub struct GenderDistribution
{
    probabilities: GenderChances,
    weights: WeightedIndex<f64>
}

impl TryFrom<GenderChances> for GenderDistribution
{
    type Error = eyre::Report;

    fn try_from(value: GenderChances) -> Result<Self, Self::Error> {
        let weights = WeightedIndex::new(value)?;

        Ok(Self {
            probabilities: value,
            weights
        })
    }
}

impl From<GenderDistribution> for GenderChances
{
    fn from(value: GenderDistribution) -> Self {
        value.probabilities
    }
}

impl Distribution<Gender> for GenderDistribution
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender
    {
        let index = self.weights.sample(rng);

        Gender::from_repr(index)
            .expect("Invalid sized weights array, which should be impossible")
    }
}
