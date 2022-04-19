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
    /// value to search pokemon by [name,number,type]
    #[clap(short,long,default_value="name")]
    pub category: String,
    /// values to filter output
    #[clap(short,long,default_value="number,name,type1,type2,catch_rate")]
    pub filter: String,
    /// input to search
    pub query: String,
}
