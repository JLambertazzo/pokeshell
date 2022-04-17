use clap::Parser;
use pokeshell::find_pokemon;
use anyhow::Result;

/// CLI App for Pokemon Data
#[derive(Parser)]
#[clap(about, version)]
struct Cli {
    /// type of information to search [pokemon, move, info]
    #[clap(short,long,default_value="pokemon")]
    field: String,
    /// attribute to search by see long about for options
    #[clap(short,long,default_value="name")]
    attr: String,
    /// What to search for
    query: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    println!("Searching for the {}: `{}`", args.field, args.query);
    if args.field == "pokemon" {
        let pokemon = find_pokemon(args.query);
        match pokemon {
            Ok(_) => println!("Found: {:?}", pokemon.unwrap()),
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(())
}
