use crate::cli::Pokemon as Args;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Pokemon {
    pub number: i32,
    pub name: String,
    pub type1: String,
    pub type2: String,
    pub catch_rate: i32,
}

const _NULL_PKMN: Pokemon = Pokemon {
    number: 0,
    name: String::new(),
    type1: String::new(),
    type2: String::new(),
    catch_rate: 0,
};

fn _find_pokemon(input: &String, category: &String) -> Result<(Pokemon, bool)> {
    let mut rdr = csv::Reader::from_path("./src/pokemon.csv")?;
    for result in rdr.deserialize() {
        let pokemon: Pokemon = result?;
        let value: Option<Pokemon> = match category {
            x if x.eq("number") => if pokemon.number.to_string() == *input { Some(pokemon) } else { None },
            x if x.eq("name") => if pokemon.name == *input { Some(pokemon) } else { None },
            x if x.eq("type") => {
                if pokemon.type1 == *input || pokemon.type2 == *input {
                    Some(pokemon)
                } else {
                    None
                }
            },
            _ => panic!("{} is not a valid category", category)
        };
        if value.is_some() {
            return Ok((value.unwrap(), true));
        }
    }
    Ok((_NULL_PKMN, false))
}

#[allow(dead_code)]
pub fn use_pokemon(args: &Args) {
    let (pokemon, success) = _find_pokemon(&(args.query),&(args.category))
        .with_context(|| "Error finding pokemon")
        .unwrap();
    if success {
        println!("Found pokemon: {:?}", pokemon);
    } else {
        println!("Pokemon {} not found.", args.query);
    }
}
