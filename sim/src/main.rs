use color_eyre::eyre::{self, OptionExt};
use directory::pokemon::manager::{PokemonManager, PokemonNames};

fn main() -> eyre::Result<()>
{
    let manager = PokemonManager::new()?;
    let sylveon = manager
        .spawn(PokemonNames::Sylveon, 50)
        .build();

    println!("{:#?}", sylveon);

    Ok(())
}
