use serde::Deserialize;
use anyhow::{Result,Error,Context};

#[derive(Debug,Deserialize)]
#[allow(dead_code)]
pub struct Pokemon {
    pub number: i32,
    pub name: String,
    pub type1: String,
    pub type2: String,
    pub catch_rate: i32,
}

pub fn find_pokemon(name: String) -> Result<Pokemon> {
    let mut rdr = csv::Reader::from_path("./src/pokemon.csv")
        .with_context(||"error opening reader")?;
    for result in rdr.deserialize() {
        let pokemon: Pokemon = result.with_context(||"Could not read line")?;
        if pokemon.name == name {
            return Ok(pokemon);
        }
    }
    Err(Error::msg("Pokemon not found"))
}