use std::{fs, sync::OnceLock};

use color_eyre::eyre::{self, eyre};
use rayon::prelude::*;
use strum::{EnumCount, VariantArray};

use crate::moves::BaseMove;

proc_macros::make_moves_enum!("assets/moves");

pub struct MoveManager
{
    moves: [BaseMove; MoveNames::COUNT],
}

impl MoveManager
{
    pub fn get() -> eyre::Result<&'static Self>
    {
        static INSTANCE: OnceLock<eyre::Result<MoveManager>> = OnceLock::new();
        INSTANCE
            .get_or_init(|| {
                let data: Vec<BaseMove> = MoveNames::VARIANTS
                    .into_par_iter()
                    .map(|name| {
                        let path = name.path();

                        let content = fs::read_to_string(path)?;
                        let data: BaseMove = toml::from_str(&content)?;

                        Ok(data)
                    })
                    .collect::<eyre::Result<Vec<BaseMove>>>()?;

                let result: [BaseMove; MoveNames::COUNT] = data
                    .try_into()
                    .map_err(|_| eyre!("Idiot Programmer somehow messed up move counts"))?;

                Ok(Self { moves: result })
            })
            .as_ref()
            .map_err(|err| eyre!("Error loading move manager: {}", err))
    }
}
