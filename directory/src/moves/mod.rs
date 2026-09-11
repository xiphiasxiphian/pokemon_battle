use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
};
use serde::{Deserialize, Serialize};

use crate::{
    moves::manager::MoveManager,
    pokemon::{Learnset, attributes::types::Type},
};

pub mod handler;
pub mod manager;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Hash)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Category
{
    Physical
    {
        power: u32,
    },
    Special
    {
        power: u32,
    },
    Status,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BaseMove
{
    name: String,
    id: String,
    move_type: Type,
    category: Category,
    accuracy: u32,
    pp: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Move<'a>
{
    base: &'a BaseMove,
    current_pp: u32,
    pp_multiplier: f64,
}

impl<'a> Move<'a>
{
    pub fn default_from_base(base: &'a BaseMove) -> Self
    {
        Self {
            base,
            current_pp: base.pp,
            pp_multiplier: 1.0,
        }
    }

    pub fn reset_pp(&mut self) { self.current_pp = (self.base.pp as f64 * self.pp_multiplier).ceil() as u32 }
}

pub type MoveList<'a> = [Option<Move<'a>>; 4];
pub struct MoveDistribution<'a>
{
    move_manager: &'static MoveManager,
    learnset: &'a Learnset,
    level: u32,
}

impl<'a> MoveDistribution<'a>
{
    pub fn new(move_manager: &'static MoveManager, learnset: &'a Learnset, level: u32) -> Self
    {
        Self {
            move_manager,
            learnset,
            level,
        }
    }
}

impl<'a> Distribution<MoveList<'a>> for MoveDistribution<'a>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MoveList<'a>
    {
        let eligible_moves: Vec<&BaseMove> = self
            .learnset
            .iter()
            .filter(|(lvl, _)| *lvl <= &self.level)
            .flat_map(|(_, m)| m.iter().map(|&x| self.move_manager.get_base(x)))
            .collect();

        let pool_size = eligible_moves.len();
        let start_idx = pool_size.saturating_sub(8);
        let mut pool = eligible_moves[start_idx..].to_vec();

        let mut result: MoveList<'a> = [None, None, None, None];

        if pool.len() <= 4
        {
            for (i, m) in pool.into_iter().enumerate()
            {
                result[i] = Some(Move::default_from_base(m));
            }
            return result;
        }

        let mut weights: Vec<u32> = (1..=pool.len() as u32).collect();
        let mut selected_count = 0;

        while selected_count < 4
        {
            let dist = WeightedIndex::new(&weights).expect("Weights should be valid");
            let chosen_idx = dist.sample(rng);

            result[selected_count] = Some(Move::default_from_base(pool[chosen_idx]));
            selected_count += 1;

            pool.remove(chosen_idx);
            weights.remove(chosen_idx);
        }

        result
    }
}
