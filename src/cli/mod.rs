use clap::{Parser,Subcommand};

#[derive(Parser)]
#[clap(author,version,about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Pokemon(Pokemon),
}

/// Searching for pokemon
#[derive(clap::Args)]
#[clap(about, version)]
pub struct Pokemon {
    /// pokemon to search for
    pub query: String,
}
