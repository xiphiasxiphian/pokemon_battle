use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
};
use strum::{EnumCount, FromRepr};

use crate::pokemon::attributes::stats::Stat;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, EnumCount, FromRepr)]
pub enum Nature
{
    #[default]
    Hardy,
    Lonely,
    Adamant,
    Naughty,
    Brave,
    Bold,
    Docile,
    Impish,
    Lax,
    Relaxed,
    Modest,
    Mild,
    Bashful,
    Rash,
    Quiet,
    Calm,
    Gentle,
    Careful,
    Quirky,
    Sassy,
    Timid,
    Hasty,
    Jolly,
    Naive,
    Serious,
}

impl Nature
{
    pub const NATURE_MODIFIER: f64 = 0.1;
    pub const NATURE_UP: f64 = 1.0 + Self::NATURE_MODIFIER;
    pub const NATURE_DOWN: f64 = 1.0 - Self::NATURE_MODIFIER;

    pub const fn stat_changes(self) -> (Stat, Stat)
    {
        const TABLE: [(Stat, Stat); 25] = [
            // +Attack
            (Stat::Attack, Stat::Attack),         // Hardy
            (Stat::Attack, Stat::Defence),        // Lonely
            (Stat::Attack, Stat::SpecialAttack),  // Adamant
            (Stat::Attack, Stat::SpecialDefence), // Naughty
            (Stat::Attack, Stat::Speed),          // Brave
            // +Defence
            (Stat::Defence, Stat::Attack),         // Bold
            (Stat::Defence, Stat::Defence),        // Docile
            (Stat::Defence, Stat::SpecialAttack),  // Impish
            (Stat::Defence, Stat::SpecialDefence), // Lax
            (Stat::Defence, Stat::Speed),          // Relaxed
            // +Sp. Atk
            (Stat::SpecialAttack, Stat::Attack),         // Modest
            (Stat::SpecialAttack, Stat::Defence),        // Mild
            (Stat::SpecialAttack, Stat::SpecialAttack),  // Bashful
            (Stat::SpecialAttack, Stat::SpecialDefence), // Rash
            (Stat::SpecialAttack, Stat::Speed),          // Quiet
            // +Sp. Def
            (Stat::SpecialDefence, Stat::Attack),         // Calm
            (Stat::SpecialDefence, Stat::Defence),        // Gentle
            (Stat::SpecialDefence, Stat::SpecialAttack),  // Careful
            (Stat::SpecialDefence, Stat::SpecialDefence), // Quirky
            (Stat::SpecialDefence, Stat::Speed),          // Sassy
            // +Speed
            (Stat::Speed, Stat::Attack),         // Timid
            (Stat::Speed, Stat::Defence),        // Hasty
            (Stat::Speed, Stat::SpecialAttack),  // Jolly
            (Stat::Speed, Stat::SpecialDefence), // Naive
            (Stat::Speed, Stat::Speed),          // Serious
        ];
        TABLE[self as usize]
    }

    pub fn get_modifier(self, stat: Stat) -> f64
    {
        if stat == Stat::Health
        {
            return 1.0;
        } // this is kinda an error case, but 1.0 just signifies "do nothing"

        let (inc, dec) = self.stat_changes();
        if inc == dec
        {
            1.0
        }
        else if stat == inc
        {
            Self::NATURE_UP
        }
        else if stat == dec
        {
            Self::NATURE_DOWN
        }
        else
        {
            1.0
        }
    }
}

impl Distribution<Nature> for StandardUniform
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Nature
    {
        let range = 0..Nature::COUNT;

        let index = rng.random_range(range);
        Nature::from_repr(index).expect("Invalid index. Check randomisation as this should be impossible otherwise")
    }
}
