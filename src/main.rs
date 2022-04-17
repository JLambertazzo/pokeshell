use clap::Parser;
use pokeshell::find_pokemon;

/// CLI App for Pokemon Data
#[derive(Parser)]
#[clap(about, version)]
struct Cli {
    /// type of information to search [pokemon, move, info]
    #[clap(short,long,default_value="pokemon")]
    field: String,
    /// What to search for
    query: String,
}

fn main() {
    let args = Cli::parse();
    println!("Searching for the {}: `{}`", args.field, args.query);
    if args.field == "pokemon" {
        let pokemon = find_pokemon(args.query).unwrap();
        println!("Found: {:?}", pokemon);
    }
}
