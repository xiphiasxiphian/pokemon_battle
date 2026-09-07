use serde::{Deserialize, Deserializer, Serialize, de::Error};
use strum::EnumCount;

#[derive(Clone, Copy, Debug, derive_more::Display, PartialEq, Eq, Hash, EnumCount, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type
{
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

pub type PokemonType = (Type, Option<Type>);

impl Type
{
    pub const MATCHUPS: [[Matchup; Self::COUNT]; Self::COUNT] = {
        use Matchup::{Double as D, Half as H, None as N, Regular as R};

        [
            //        NOR FIR WAT ELE GRA ICE FIG POI GRO FLY PSY BUG ROC GHO DRA DAR STE FAI
            /* NOR */
            [R, R, R, R, R, R, R, R, R, R, R, R, H, N, R, R, H, R],
            /* FIR */ [R, H, H, R, D, D, R, R, R, R, R, D, H, R, H, R, D, R],
            /* WAT */ [R, D, H, R, H, R, R, R, D, R, R, R, D, R, H, R, R, R],
            /* ELE */ [R, R, D, H, H, R, R, R, N, D, R, R, R, R, H, R, R, R],
            /* GRA */ [R, H, D, R, H, R, R, H, D, H, R, H, D, R, H, R, H, R],
            /* ICE */ [R, H, H, R, D, H, R, R, D, D, R, R, R, R, D, R, H, R],
            /* FIG */ [D, R, R, R, R, D, R, H, R, H, H, H, D, N, R, D, D, H],
            /* POI */ [R, R, R, R, D, R, R, H, H, R, R, R, H, H, R, R, N, D],
            /* GRO */ [R, D, R, D, H, R, R, D, R, N, R, H, D, R, R, R, D, R],
            /* FLY */ [R, R, R, H, D, R, D, R, R, R, R, D, H, R, R, R, H, R],
            /* PSY */ [R, R, R, R, R, R, D, D, R, R, H, R, R, R, R, N, H, R],
            /* BUG */ [R, H, R, R, D, R, H, H, R, H, D, R, R, H, R, D, H, H],
            /* ROC */ [R, D, R, R, R, D, H, R, H, D, R, D, R, R, R, R, H, R],
            /* GHO */ [N, R, R, R, R, R, R, R, R, R, D, R, R, D, R, H, R, R],
            /* DRA */ [R, R, R, R, R, R, R, R, R, R, R, R, R, R, D, R, H, N],
            /* DAR */ [R, R, R, R, R, R, H, R, R, R, D, R, R, D, R, H, R, H],
            /* STE */ [R, H, H, H, R, D, R, R, R, R, R, R, D, R, R, R, H, D],
            /* FAI */ [R, H, R, R, R, R, D, H, R, R, R, R, R, R, D, D, H, R],
        ]
    };

    pub fn matchup(self, other: Self) -> Matchup { Self::MATCHUPS[self as usize][other as usize] }

    pub fn deserialize_typing<'de, D>(deserializer: D) -> Result<PokemonType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let types = Vec::<Type>::deserialize(deserializer)?;

        match types.len()
        {
            1 => Ok((types[0], None)),
            2 => Ok((types[0], Some(types[1]))),
            _ => Err(D::Error::custom("typing array must contain exactly 1 or 2 types")),
        }
    }

    pub fn multiplier_against(self, target: PokemonType) -> f64
    {
        self.matchup(target.0).multiplier() * target.1.map(|x| self.matchup(x).multiplier()).unwrap_or(1.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Matchup
{
    None,
    Half,
    Regular,
    Double,
}

impl Matchup
{
    pub fn multiplier(self) -> f64
    {
        match self
        {
            Self::None => 0.0,
            Self::Half => 0.5,
            Self::Regular => 1.0,
            Self::Double => 2.0,
        }
    }
}
