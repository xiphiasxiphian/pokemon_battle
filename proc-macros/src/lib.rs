use std::{fs, path::PathBuf};

use heck::ToPascalCase;
use jwalk::WalkDir;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use rayon::prelude::*;
use serde::Deserialize;
use syn::{LitStr, parse_macro_input};

const FILE_FORMAT: &'static str = "toml";

#[derive(Deserialize)]
struct PokemonFile
{
    id: String, // other fields arent important here
}

#[proc_macro]
pub fn make_pokemon_enum(input: TokenStream) -> TokenStream
{
    let dir_lit = parse_macro_input!(input as LitStr);
    let relative_path = dir_lit.value();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let target_dir = PathBuf::from(manifest_dir).join(&relative_path);

    let (ids_strings, paths): (Vec<String>, Vec<String>) = WalkDir::new(&target_dir)
        .into_iter()
        .par_bridge()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();

            if !(path.is_file() && path.extension()? == FILE_FORMAT)
            {
                return None;
            }
            Some(path)
        })
        .fold(
            || (vec![], vec![]),
            |(mut names, mut paths), path| {
                let content = fs::read_to_string(&path)
                    .unwrap_or_else(|err| panic!("Failed to read file at {}: {}", path.display(), err));
                let config: PokemonFile = toml::from_str(&content)
                    .unwrap_or_else(|err| panic!("Failed to parse file at {}: {}", path.display(), err));

                let ident_str = config.id.to_pascal_case();

                names.push(ident_str);
                paths.push(
                    path.into_os_string()
                        .into_string()
                        .expect("Failed to convert OsString into string"),
                );
                (names, paths)
            },
        )
        .reduce(
            || (vec![], vec![]),
            |(mut names, mut paths), (in_names, in_path)| {
                names.extend(in_names);
                paths.extend(in_path);

                (names, paths)
            },
        );

    let ids: Vec<Ident> = ids_strings.iter().map(|x| Ident::new(x, Span::call_site())).collect();
    if ids.is_empty()
    {
        panic!("Couldn't find any pokemon config files in {}", target_dir.display())
    }

    let expanded = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, strum::EnumCount, strum::EnumIter, strum::VariantArray)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        pub enum PokemonNames
        {
            #( #ids, )*
        }

        impl PokemonNames
        {
            pub fn path(&self) -> &'static ::std::path::Path
            {
                match self
                {
                    #( Self::#ids => ::std::path::Path::new(#paths), )*
                }
            }
        }
    };

    expanded.into()
}
