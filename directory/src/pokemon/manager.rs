use std::{collections::{HashMap, hash_map::Entry}, fs::{self}, path::Path};

use color_eyre::eyre::{self, eyre};
use jwalk::WalkDir;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::pokemon::{BasePokemon, Pokemon, PokemonBuilder};

#[derive(Debug)]
pub struct PokemonManager
{
    pokemon: HashMap<String, BasePokemon>,
}

impl PokemonManager
{
    const FILE_FORMAT: &'static str = "toml";

    pub fn new(path: &'static Path) -> eyre::Result<Self>
    {
        let results: HashMap<String, BasePokemon> = WalkDir::new(path)
            .into_iter()
            .par_bridge()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();

                if !(path.is_file() && path.extension()? == Self::FILE_FORMAT) { return None }
                Some(path)
            })
            .try_fold(
                || HashMap::new(),
                |mut local_map, path| {
                    let content = fs::read_to_string(&path)?;
                    let data = toml::from_str::<BasePokemon>(&content)?;

                    match local_map.entry(data.id.clone())
                    {
                        Entry::Occupied(_) => {
                            return Err(eyre!("Failed to import pokemon at {:?} as ID already exists", path));
                        }
                        Entry::Vacant(e) => {
                            e.insert_entry(data);
                        }
                    }

                    Ok(local_map)
                }
            )
            .try_reduce(
                || HashMap::new(),
                |mut map1, map2| {
                    map1.extend(map2);
                    Ok(map1)
                }
            )?;

        Ok(
            Self { pokemon: results }
        )
    }

    pub fn spawn_random<'a, 'b>(&'a self, id: &str, level: u32) -> Option<Pokemon<'b>>
    where 'a: 'b
    {
        self.pokemon.get(id).map(|x| Pokemon::builder(x, level).build())
    }

    pub fn spawn<'a, 'b>(&'a self, id: &str, level: u32) -> Option<PokemonBuilder<'b>>
    where 'a: 'b
    {
        self.pokemon.get(id).map(|x| Pokemon::builder(x, level))
    }
}
