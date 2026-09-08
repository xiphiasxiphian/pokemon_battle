use std::{fs, path::{Path, PathBuf}};

use heck::ToPascalCase;
use jwalk::WalkDir;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use rayon::prelude::*;
use serde::{Deserialize, de::DeserializeOwned};
use syn::{LitStr, parse_macro_input};

const FILE_FORMAT: &'static str = "toml";

#[derive(Deserialize)]
struct PokemonFile
{
    id: String, // other fields arent important here
}

#[derive(Deserialize)]
struct MoveFile
{
    id: String,
}

#[proc_macro]
pub fn make_pokemon_enum(input: TokenStream) -> TokenStream
{
    make_enum_helper::<_, PokemonFile>(
        input,
        syn::Ident::new("PokemonNames", Span::call_site()),
        |x| x.iter().map(|x| Ident::new(&x.id.to_pascal_case(), Span::call_site())).collect()
    )
}

#[proc_macro]
pub fn make_moves_enum(input: TokenStream) -> TokenStream
{
    make_enum_helper::<_, MoveFile>(
        input,
        syn::Ident::new("MoveNames", Span::call_site()),
        |x| x.iter().map(|x| Ident::new(&x.id.to_pascal_case(), Span::call_site())).collect()
    )}

fn make_enum_helper<F, T: DeserializeOwned + Send + Sync>(input: TokenStream, name: syn::Ident, func: F) -> TokenStream
where
    F: FnOnce(Vec<T>) -> Vec<Ident>
{
    let dir_lit = parse_macro_input!(input as LitStr);
    let relative_path = dir_lit.value();

    let (data, paths) = get_file_info::<T>(Path::new(&relative_path));
    let ids: Vec<Ident> = func(data);

    let expanded = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, strum::EnumCount, strum::EnumIter, strum::VariantArray)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        pub enum #name
        {
            #( #ids, )*
        }

        impl #name
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

fn get_file_info<T: DeserializeOwned + Send + Sync>(path: &Path) -> (Vec<T>, Vec<String>)
{
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let target_dir = PathBuf::from(manifest_dir).join(path);

    let (data, paths): (Vec<T>, Vec<String>) = WalkDir::new(&target_dir)
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
                let config: T = toml::from_str(&content)
                    .unwrap_or_else(|err| panic!("Failed to parse file at {}: {}", path.display(), err));

                names.push(config);
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

    if data.is_empty() { panic!("Failed to find any files") }
    (data, paths)
}
