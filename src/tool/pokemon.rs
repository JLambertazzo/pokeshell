use crate::cli::Pokemon as Args;
use anyhow::Result;
use serde::Deserialize;
use prettytable::{Table, cell, row, format};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Pokemon {
    pub number: i32,
    pub name: String,
    pub type1: String,
    pub type2: String,
    pub catch_rate: i32,
}

impl Default for Pokemon {
    fn default() -> Self {
        Pokemon {
            number: 0,
            name: String::new(),
            type1: String::new(),
            type2: String::new(),
            catch_rate: 0,
        }
    }
}

fn _find_pokemon(input: &String, category: &String) -> Result<Vec<Pokemon>> {
    let rdr = csv::Reader::from_path("./src/pokemon.csv")?;
    let vect = rdr.into_deserialize()
        .filter_map(|r: Result<Pokemon,csv::Error>| r.ok())
        .filter(|res| {
            match category {
                x if x.eq("number") => res.number.to_string() == *input,
                x if x.eq("name") => res.name == *input,
                x if x.eq("type") => res.type1 == *input || res.type2 == *input,
                _ => panic!("{} is not a valid category", category)
            }
        }).collect();
    Ok(vect)
}

fn _print_pokemon(pokemon: Vec<Pokemon>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["DEX#","NAME","TYPE1","TYPE2","CATCH_RATE"]);
    for p in pokemon {
        table.add_row(row![
            p.number.to_string(),
            p.name,
            p.type1,
            p.type2,
            p.catch_rate.to_string(),
        ]);
    }
    table.printstd();
}

#[allow(dead_code)]
pub fn use_pokemon(args: &Args) {
    let pokemon = _find_pokemon(&(args.query),&(args.category))
        .unwrap_or_default();
    if pokemon.len() > 0 {
        _print_pokemon(pokemon);
    } else {
        println!("Pokemon {} not found.", args.query);
    }
}
