use color_eyre::eyre::{self};
use directory::pokemon::manager::{PokemonManager, PokemonNames};

fn main() -> eyre::Result<()>
{
    let manager = PokemonManager::get()?;
    let sylveon = manager.spawn(PokemonNames::Sylveon, 50).build();

    println!("{:#?}", sylveon);

    Ok(())
}
