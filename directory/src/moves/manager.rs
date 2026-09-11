use std::{fs, sync::OnceLock};

use color_eyre::eyre::{self, eyre};
use rayon::prelude::*;
use strum::{EnumCount, VariantArray};

use crate::moves::{BaseMove, Move};

proc_macros::make_moves_enum!("assets/moves");

pub struct MoveManager
{
    moves: [BaseMove; MoveNames::COUNT],
}

impl MoveManager
{
    pub fn get() -> &'static Self
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
            .unwrap_or_else(|err| panic!("Failed to init Move Manager: {}", err))

        // panic on failure given that this is a critical init component.
        // without it, any system that relies on it will fail, and at least one of them
        // will just have to panic.
    }

    pub fn get_base(&self, id: MoveNames) -> &BaseMove { &self.moves[id as usize] }

    pub fn spawn<'a, 'b>(&'a self, id: MoveNames) -> Move<'b>
    where
        'a: 'b,
    {
        let base = self.get_base(id);
        Move::default_from_base(base)
    }
}
