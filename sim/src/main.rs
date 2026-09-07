use std::path::Path;

use color_eyre::eyre::{self, OptionExt};
use directory::pokemon::manager::PokemonManager;

fn main() -> eyre::Result<()>
{
    let manager = PokemonManager::new(Path::new("./assets/pokemon/"))?;
    let sylveon = manager
        .spawn("SYLVEON", 50)
        .ok_or_eyre("Failed to get Sylveon")?
        .build();

    println!("{:#?}", sylveon);

    Ok(())
}
