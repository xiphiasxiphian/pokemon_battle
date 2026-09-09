use serde::{Deserialize, Serialize};
use strum::{EnumCount, VariantArray};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, strum::EnumCount, strum::VariantArray)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GrowthRate
{
    Fluctuating,
    Slow,
    MediumSlow,
    MediumFast,
    Fast,
    Erratic,
}

impl GrowthRate
{
    /// using size 101 so the index matches the level exactly. i may regret this decision idk
    pub const EXP_TABLE: [[u32; 101]; GrowthRate::COUNT] = Self::compute_exp_table();

    pub fn level(self, experience: u32) -> u32
    {
        let row = Self::EXP_TABLE[self as usize];
        match row.binary_search(&experience)
        {
            Ok(index) => index,
            Err(index) => index - 1, // this is fine because index cannot be 0, as the first arm of the match would trigger
        }
        .try_into()
        .expect("Somehow got a level outside the range of a u32. This shouldnt be possible")
    }

    pub fn experience_for_level(self, level: u32) -> u32 { Self::EXP_TABLE[self as usize][level as usize] }

    /// Compute table for experience requirements.
    /// Formulas taken from https://pokestats.gg/growth-rates
    const fn compute_exp_table() -> [[u32; 101]; GrowthRate::COUNT]
    {
        let mut table = [[0; 101]; GrowthRate::COUNT];
        let mut rate_index = 0;

        while rate_index < GrowthRate::COUNT
        {
            // level "0" and 1 always require 0 exp. this prevents
            // gen 1 underflow bug for medium slow growth
            let mut level: u32 = 2;

            // would love to use array::from_fn here but its not stable in a const context yet
            while level <= 100
            {
                let l3 = level.pow(3);
                let l2 = level.pow(2);

                let exp = match Self::VARIANTS[rate_index]
                {
                    GrowthRate::Fast => (4 * l3) / 5,
                    GrowthRate::MediumFast => l3,
                    GrowthRate::MediumSlow =>
                    {
                        let positive = (6 * l3) / 5 + 100 * level;
                        let negative = 15 * l2 + 140;
                        positive.saturating_sub(negative)
                    }
                    GrowthRate::Slow => (5 * l3) / 4,

                    GrowthRate::Erratic if level <= 50 => (l3 * (100 - level)) / 50,
                    GrowthRate::Erratic if level <= 68 => (l3 * (150 - level)) / 100,
                    GrowthRate::Erratic if level <= 98 => (l3 * ((1911 - 10 * level) / 3)) / 500,
                    GrowthRate::Erratic => (l3 * (160 - level)) / 100,

                    GrowthRate::Fluctuating if level <= 15 => (l3 * (((level + 1) / 3) + 24)) / 50,
                    GrowthRate::Fluctuating if level <= 36 => (l3 * (level + 14)) / 50,
                    GrowthRate::Fluctuating => (l3 * ((level / 2) + 32)) / 50,
                };

                table[rate_index][level as usize] = exp;
                level += 1;
            }
            rate_index += 1;
        }

        table
    }
}
