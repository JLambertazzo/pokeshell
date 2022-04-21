use clap::Parser;
mod cli;
use cli::{Cli,Commands};
mod tool;
use tool::pokemon;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Pokemon(args) => pokemon::use_pokemon(args),
    }
}
