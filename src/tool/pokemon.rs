use crate::cli::Pokemon as Args;
use anyhow::Result;
use serde::Deserialize;
use prettytable::{Table, format, Row, Cell};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Pokemon {
    pub number: i32,
    pub name: String,
    pub type1: String,
    pub type2: String,
    pub catch_rate: i32,
}

impl Pokemon {
    const PKMN_COLS: [&'static str;5] = [
        "number",
        "name",
        "type1",
        "type2",
        "catch_rate"
    ];
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

fn _get_filtered_cells(vec: [String;5], filter: &String) -> Vec<Cell> {
    let mut result: Vec<Cell> = Vec::new();
    for (i,col) in Pokemon::PKMN_COLS.into_iter().enumerate() {
        if filter == "all" || filter.contains(col) {
            result.push(Cell::new(vec[i].as_str()));
        }
    }
    result
}

fn _print_pokemon(pokemon: Vec<Pokemon>, filter: &String) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(Row::new(_get_filtered_cells(
        ["DEX#".to_string(),"NAME".to_string(),"TYPE1".to_string(),"TYPE2".to_string(),"CATCH_RATE".to_string()],
        filter
    )));
    for p in pokemon {
        table.add_row(Row::new(_get_filtered_cells(
            [
                p.number.to_string(),
                p.name,
                p.type1,
                p.type2,
                p.catch_rate.to_string(),
            ],
            filter
        )));
    }

    table.printstd();
}

#[allow(dead_code)]
pub fn use_pokemon(args: &Args) {
    let pokemon = _find_pokemon(&(args.query),&(args.category))
        .unwrap_or_default();
    if pokemon.len() > 0 {
        _print_pokemon(pokemon, &args.filter);
    } else {
        println!("Pokemon {} not found.", args.query);
    }
}
