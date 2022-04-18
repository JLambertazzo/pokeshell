use clap::Parser;
mod cli;
mod tool;
use tool::pokemon;

/// CLI App for Pokemon Data
#[derive(Parser)]
#[clap(about, version)]
struct RootCli {
    /// type of information to search [pokemon, move]
    collection: String,
}

fn main() {
    let args = RootCli::parse();
    match args.collection {
        x if x.eq("pokemon") => pokemon::use_pokemon(),
        _ => {}
    }
}
