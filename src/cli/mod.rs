use clap::Parser;

#[derive(Parser)]
pub enum Cli {
    Pokemon(Pokemon),
}

/// Searching for pokemon
#[derive(clap::Args)]
#[clap(about, version)]
pub struct Pokemon {
    /// pokemon to search for
    pub query: String,
}
