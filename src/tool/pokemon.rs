use crate::cli::Cli;
use anyhow::{Context, Result};
use clap::Parser;
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

fn _find_pokemon(name: &String) -> Result<(Pokemon, bool)> {
    let mut rdr = csv::Reader::from_path("./src/pokemon.csv")?;
    for result in rdr.deserialize() {
        let pokemon: Pokemon = result?;
        if pokemon.name == *name {
            return Ok((pokemon, true));
        }
    }
    Ok((_NULL_PKMN, false))
}

#[allow(dead_code)]
pub fn use_pokemon() {
    let Cli::Pokemon(args) = Cli::parse();
    let (pokemon, success) = _find_pokemon(&args.query)
        .with_context(|| "Error finding pokemon")
        .unwrap();
    if success {
        println!("Found pokemon: {:?}", pokemon);
    } else {
        println!("Pokemon {} not found.", args.query);
    }
}
