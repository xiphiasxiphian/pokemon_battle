use serde::{Deserialize, Serialize};

use crate::pokemon::attributes::types::Type;

pub mod manager;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Hash, )]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Category
{
    Physical { power: u32 },
    Special { power: u32 },
    Status
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

#[derive()]
pub struct Move<'a>
{
    base: &'a BaseMove,
    current_pp: u32,
    pp_multiplier: f64,
}
