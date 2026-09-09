use color_eyre::eyre::{self};
use directory::{moves::manager::MoveManager, pokemon::manager::{PokemonManager, PokemonNames}};

fn main() -> eyre::Result<()>
{
    let move_manager = MoveManager::get()?;

    let pokemon_manager = PokemonManager::get()?;
    let sylveon = pokemon_manager.spawn(PokemonNames::Sylveon, 50).build();

    println!("{:#?}", sylveon);

    Ok(())
}
