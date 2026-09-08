use std::{
    fs::{self},
    sync::OnceLock,
};

use color_eyre::eyre::{self, eyre};
use rayon::prelude::*;
use strum::{EnumCount, VariantArray};

use crate::pokemon::{BasePokemon, Pokemon, builder::PokemonBuilder};

proc_macros::make_pokemon_enum!("assets/pokemon");

#[derive(Debug)]
pub struct PokemonManager
{
    pokemon: [BasePokemon; PokemonNames::COUNT],
}

impl PokemonManager
{
    pub fn get() -> eyre::Result<&'static Self>
    {
        static INSTANCE: OnceLock<eyre::Result<PokemonManager>> = OnceLock::new();
        INSTANCE
            .get_or_init(|| {
                let data: Vec<BasePokemon> = PokemonNames::VARIANTS
                    .into_par_iter()
                    .map(|name| {
                        let path = name.path();

                        let content = fs::read_to_string(path)?;
                        let data: BasePokemon = toml::from_str(&content)?;

                        Ok(data)
                    })
                    .collect::<eyre::Result<Vec<BasePokemon>>>()?;

                let result: [BasePokemon; PokemonNames::COUNT] = data
                    .try_into()
                    .map_err(|_| eyre!("Idiot Programmer somehow messed up pokemon counts"))?;

                Ok(Self { pokemon: result })
            })
            .as_ref()
            .map_err(|err| eyre!("Error loading pokemon manager: {}", err))
    }

    pub fn spawn<'a, 'b>(&'a self, id: PokemonNames, level: u32) -> PokemonBuilder<'b>
    where
        'a: 'b,
    {
        Pokemon::builder_from_level(&self.pokemon[id as usize], level)
    }

    pub fn spawn_from_experience<'a, 'b>(&'a self, id: PokemonNames, experience: u32) -> PokemonBuilder<'b>
    where
        'a: 'b,
    {
        Pokemon::builder(&self.pokemon[id as usize], experience)
    }
}
