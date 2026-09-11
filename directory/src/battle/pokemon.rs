use std::process::id;

use strum::{EnumCount, VariantArray};

use crate::pokemon::{Pokemon, attributes::stats::{Stat, Stats}};

const STAGE_BOUND: i8 = 6;
pub type StatStage = i8;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Status
{
    Frozen,
    Paralyzed,
    Burned,
    Asleep,
    Confused,
}

pub struct BattlePokemon<'a, 'p, 'm>
where
    'p: 'a,
    'm: 'a,
{
    base: &'a mut Pokemon<'p, 'm>,
    stat_stages: Stats<StatStage>,
    accuracy: StatStage,
    evasion: StatStage,
    status: Option<Status>,
}

impl<'a, 'p, 'm> BattlePokemon<'a, 'p, 'm>
{
    pub fn stat(&self, stat: Stat) -> u32
    {
        let base = self.base.stat(stat) as f64;
        let multiplier = self.stat_multiplier(stat);

        (base * multiplier).ceil() as u32
    }

    pub fn stats(&self) -> Stats
    {
        let base = self.base.stats().map(|x| x as f64);
        let multipliers = self.stat_stages.map(|x| Self::stage_to_multipler(x, 2.0));

        (base * multipliers).map(|x| x as u32)
    }

    pub fn modify_stat_stage<F>(&mut self, stat: Stat, action: F)
    where
        F: FnOnce(i8) -> i8
    {
        let new_value = action(self.stat_stages.stat(stat)).clamp(-STAGE_BOUND, STAGE_BOUND);
        self.stat_stages.set_stat(stat, new_value);
    }

    // accuracy and evasion are handled separately, given that
    // a) they are battle only stats
    // b) their multipliers are treated differently
    // c) they exist only as multipliers

    pub fn accuracy(&self) -> f64
    {
        Self::stage_to_multipler(self.accuracy, 3.0)
    }

    pub fn evasion(&self) -> f64
    {
        // evasion is just accuracy flipped
        Self::stage_to_multipler(-self.evasion, 3.0)
    }

    fn stat_multiplier(&self, stat: Stat) -> f64
    {
        let stage = self.stat_stages.stat(stat);
        Self::stage_to_multipler(stage, 2.0)
    }

    fn stage_to_multipler(stage: StatStage, base: f64) -> f64
    {
        if stage < 0
        {
            base / (stage.abs() as f64 + base)
        }
        else
        {
            (stage.abs() as f64 + base) / base
        }
    }

    fn status_multiplier(&self) -> Stats<f64>
    {
        (&self.status).map(|x| match x {
            Status::Burned => Stats::from_iter([(Stat::Attack, 0.5)]),
            Status::Paralyzed => Stats::from_iter([(Stat::Speed, 0.5)]),
            _ => Stats::new([1.0; Stat::COUNT]),
        })
        .unwrap_or_else(|| Stats::new([1.0; Stat::COUNT]))
    }
}
