use serde::Deserialize;

#[derive(Debug,Deserialize)]
#[allow(dead_code)]
pub struct Pokemon {
    number: i32,
    name: String,
    type1: String,
    type2: String,
    catch_rate: i32,
}

pub fn find_pokemon(name: String) -> Result<Pokemon,String> {
    let mut rdr = csv::Reader::from_path("./src/pokemon.csv")
        .expect("error opening reader");
    for result in rdr.deserialize() {
        let pokemon: Pokemon = result.expect("Could not read line");
        if pokemon.name == name {
            return Ok(pokemon);
        }
    }
    Err(String::from("Not found"))
}